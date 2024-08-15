// Importa los crates necesarios para definir macros en Rust.
extern crate proc_macro;

// Importa las dependencias necesarias para trabajar con macros en Rust.
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// Define la macro derivada `CrudOperations`.
// Esta macro generará métodos CRUD para la estructura a la que se aplica.
#[proc_macro_derive(CrudOperations)]
pub fn crud_operations_derive(input: TokenStream) -> TokenStream {
    // Analiza el TokenStream de entrada para obtener la definición de la estructura.
    let input = parse_macro_input!(input as DeriveInput);

    // Obtiene el identificador (nombre) de la estructura a la que se aplica la macro.
    let name = &input.ident;

    // Usa `quote` para generar el código Rust que implementa los métodos CRUD para la estructura.
    // `quote!` permite interpolar el identificador de la estructura (`#name`) en el código generado.
    let expanded = quote! {
        impl #name {
            // Método para crear una instancia de la estructura.
            // Imprime un mensaje indicando que se está creando la instancia.
            pub fn create(&self) {
                println!("Creating {:?}", self);
            }

            // Método para leer una entidad por su ID.
            // Imprime un mensaje con el ID de la entidad que se está leyendo.
            pub fn read(id: u32) {
                println!("Reading entity with id: {}", id);
            }

            // Método para actualizar la instancia de la estructura.
            // Imprime un mensaje indicando que se está actualizando la instancia.
            pub fn update(&self) {
                println!("Updating {:?}", self);
            }

            // Método para eliminar una entidad por su ID.
            // Imprime un mensaje con el ID de la entidad que se está eliminando.
            pub fn delete(id: u32) {
                println!("Deleting entity with id: {}", id);
            }
        }
    };

    // Convierte el código generado en un TokenStream para que el compilador lo use.
    TokenStream::from(expanded)
}
