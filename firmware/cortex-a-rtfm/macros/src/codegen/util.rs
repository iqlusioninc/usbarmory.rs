use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use rtfm_syntax::{ast::App, Context};
use syn::{Attribute, LitInt, PatType};

// Because we are using priority compression these mappings are trivial

/// Goes from SGI number to the priority it should run at
pub fn sgi2prio(sgi: u8) -> u8 {
    sgi + 1
}

/// Goes from dispatcher priority to the "index" of the SGI that will act as the dispatcher
pub fn prio2sgi(logical: u8) -> u8 {
    logical - 1
}

// TODO add proper implementation (needs changes in usbarmory-rt)
pub fn link_section_uninit() -> TokenStream2 {
    quote!()
    // static INDEX: AtomicUsize = AtomicUsize::new(0);

    // let index = INDEX.fetch_add(1, Ordering::Relaxed);
    // let section = format!(".uninit.rtfm{}", index);

    // quote!(#[link_section = #section])
}

/// Turns `capacity` into an unsuffixed integer literal
pub fn capacity_literal(capacity: u8) -> LitInt {
    LitInt::new(&capacity.to_string(), Span::call_site())
}

/// Turns `capacity` into a type-level (`typenum`) integer
pub fn capacity_typenum(capacity: u8, round_up_to_power_of_two: bool) -> TokenStream2 {
    let capacity = if round_up_to_power_of_two {
        capacity.checked_next_power_of_two().expect("UNREACHABLE")
    } else {
        capacity
    };

    let ident = Ident::new(&format!("U{}", capacity), Span::call_site());

    quote!(rtfm::export::consts::#ident)
}

/// Identifier for the free queue
///
/// There may be more than one free queue per task because we need one for each sender core so we
/// include the sender (e.g. `S0`) in the name
pub fn fq_ident(task: &Ident) -> Ident {
    Ident::new(&format!("{}_FQ", task.to_string()), Span::call_site())
}

/// Generates an identifier for the `INPUTS` buffer (`spawn` & `schedule` API)
pub fn inputs_ident(task: &Ident) -> Ident {
    Ident::new(&format!("{}_INPUTS", task), Span::call_site())
}

/// Generates a pre-reexport identifier for the "late resources" struct
pub fn late_resources_ident(init: &Ident) -> Ident {
    Ident::new(
        &format!("{}LateResources", init.to_string()),
        Span::call_site(),
    )
}

/// Generates a pre-reexport identifier for the "locals" struct
pub fn locals_ident(ctxt: Context<'_>, app: &App) -> Ident {
    let mut s = match ctxt {
        Context::Init(core) => app.inits[&core].name.to_string(),
        Context::Idle(core) => app.idles[&core].name.to_string(),
        Context::HardwareTask(ident) | Context::SoftwareTask(ident) => ident.to_string(),
    };

    s.push_str("Locals");

    Ident::new(&s, Span::call_site())
}

/// Generates a pre-reexport identifier for the "resources" struct
pub fn resources_ident(ctxt: Context<'_>, app: &App) -> Ident {
    let mut s = match ctxt {
        Context::Init(core) => app.inits[&core].name.to_string(),
        Context::Idle(core) => app.idles[&core].name.to_string(),
        Context::HardwareTask(ident) | Context::SoftwareTask(ident) => ident.to_string(),
    };

    s.push_str("Resources");

    Ident::new(&s, Span::call_site())
}

/// Generates an identifier for a ready queue
pub fn rq_ident(priority: u8) -> Ident {
    Ident::new(&format!("RQ{}", priority), Span::call_site())
}

/// Generates an identifier for a SGI handler
pub fn sgi_ident(i: u8) -> Ident {
    Ident::new(&format!("SGI{}", i), Span::call_site())
}

/// Generates an identifier for the `enum` of `spawn`-able tasks
///
/// This identifier needs the same structure as the `RQ` identifier because
/// there's one ready queue for each of these `T` enums
pub fn spawn_t_ident(priority: u8) -> Ident {
    Ident::new(&format!("T{}", priority), Span::call_site())
}

/// Generates an identifier for a "spawn" function
///
/// The methods of the `Spawn` structs invoke these functions. As one task may be `spawn`-ed by
/// different cores we need one "spawn" function per possible task-sender pair
pub fn spawn_ident(name: &Ident) -> Ident {
    Ident::new(&format!("spawn_{}", name.to_string()), Span::call_site())
}

/// Generates a `Mutex` implementation
pub fn impl_mutex(
    cfgs: &[Attribute],
    resources_prefix: bool,
    name: &Ident,
    ty: TokenStream2,
    ceiling: u8,
    ptr: TokenStream2,
) -> TokenStream2 {
    let (path, priority) = if resources_prefix {
        (quote!(resources::#name), quote!(self.priority()))
    } else {
        (quote!(#name), quote!(self.priority))
    };

    quote!(
        #(#cfgs)*
        impl<'a> rtfm::Mutex for #path<'a> {
            type T = #ty;

            #[inline(always)]
            fn lock<R>(&mut self, f: impl FnOnce(&mut #ty) -> R) -> R {
                /// Priority ceiling
                const CEILING: u8 = #ceiling;

                unsafe {
                    rtfm::export::lock(
                        #ptr,
                        #priority,
                        CEILING,
                        f,
                    )
                }
            }
        }
    )
}

// Regroups the inputs of a task
//
// `inputs` could be &[`input: Foo`] OR &[`mut x: i32`, `ref y: i64`]
pub fn regroup_inputs(
    inputs: &[PatType],
) -> (
    // args e.g. &[`_0`],  &[`_0: i32`, `_1: i64`]
    Vec<TokenStream2>,
    // tupled e.g. `_0`, `(_0, _1)`
    TokenStream2,
    // untupled e.g. &[`_0`], &[`_0`, `_1`]
    Vec<TokenStream2>,
    // ty e.g. `Foo`, `(i32, i64)`
    TokenStream2,
) {
    if inputs.len() == 1 {
        let ty = &inputs[0].ty;

        (
            vec![quote!(_0: #ty)],
            quote!(_0),
            vec![quote!(_0)],
            quote!(#ty),
        )
    } else {
        let mut args = vec![];
        let mut pats = vec![];
        let mut tys = vec![];

        for (i, input) in inputs.iter().enumerate() {
            let i = Ident::new(&format!("_{}", i), Span::call_site());
            let ty = &input.ty;

            args.push(quote!(#i: #ty));

            pats.push(quote!(#i));

            tys.push(quote!(#ty));
        }

        let tupled = {
            let pats = pats.clone();
            quote!((#(#pats,)*))
        };
        let ty = quote!((#(#tys,)*));
        (args, tupled, pats, ty)
    }
}
