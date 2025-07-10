#![no_std]


use soroban_sdk::{contract, contractimpl,  Env, symbol_short, Symbol};
const RESULTADO: Symbol = symbol_short!("RESULTADO");
#[contract]
pub struct Contract;


#[contractimpl]
impl Contract {
    
    pub fn sumar(env: Env, a: i128, b: i128) -> i128 {
        // Calculamos el resultado de la suma
        let resultado = a + b;
        
        // Almacenamos el resultado en el storage del contrato
        env.storage().instance().set(&RESULTADO, &resultado);
        
        // Extendemos el TTL (Time To Live) del storage
        env.storage().instance().extend_ttl(50, 100);
        
        // Devolvemos el resultado
        resultado
    }

    pub fn sumar_acumulado(env: Env, a: i128, b: i128) -> i128 {
        let resultado = a + b;
        
        // Obtener la lista existente o crear una nueva
        let mut resultados: Vec<i128> = env.storage()
            .instance()
            .get(&RESULTADOS)
            .unwrap_or(Ok(Vec::new(&env)))
            .unwrap();
        
        // Agregar el nuevo resultado
        resultados.push_back(resultado);
        
        // Guardar la lista actualizada
        env.storage().instance().set(&RESULTADOS, &resultados);
        env.storage().instance().extend_ttl(50, 100);
        
        resultado
    }

    pub fn resultado_anterior(env: Env) -> i128 {
        // Obtenemos el valor almacenado, o devolvemos 0 si no existe
        env.storage()
           .instance()
           .get(&RESULTADO)
           .unwrap_or(0) // Valor por defecto si no hay dato almacenado
    }

    // Nueva función para obtener todos los resultados
    pub fn obtener_resultados(env: Env) -> Vec<i128> {
        env.storage()
            .instance()
            .get(&RESULTADOS)
            .unwrap_or(Ok(Vec::new(&env)))
            .unwrap()
    }
}

mod test;
