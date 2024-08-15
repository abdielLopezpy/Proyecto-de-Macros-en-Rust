// Importa la macro `CrudOperations` generada en la crate `my_macro_crate`.
use my_macro_crate::CrudOperations;

// Deriva la macro `CrudOperations` para generar automáticamente los métodos CRUD
// para la estructura `User`. También se deriva `Debug` para habilitar la impresión
// formateada de la estructura con `{:?}`.
#[derive(CrudOperations, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn main() {
    // Crea una nueva instancia de `User`.
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    // Llama al método `create` generado por la macro. Este método imprime
    // la información del usuario al crearlo.
    user.create();

    // Llama al método `read`, que toma un ID como argumento e imprime la
    // información del usuario correspondiente al ID especificado.
    User::read(1);

    // Llama al método `update` que imprime un mensaje indicando que se ha
    // actualizado la información del usuario.
    user.update();

    // Llama al método `delete` que elimina al usuario con el ID especificado.
    User::delete(1);
}
