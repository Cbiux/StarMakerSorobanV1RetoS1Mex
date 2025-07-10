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

    pub fn resultado_anterior(env: Env) -> i128 {
        // Obtenemos el valor almacenado, o devolvemos 0 si no existe
        env.storage()
           .instance()
           .get(&RESULTADO)
           .unwrap_or(0) // Valor por defecto si no hay dato almacenado
    pub fn sumar(env: Env, a:i128, b:i128) -> i128 {
      //Implementar función que sume dos números
      return 20;
    }

    pub fn resultado_anterior(env: Env) -> i128 {
           //Implementar función que retorne el valor anterior
            return 20;
    }
}

mod test;
