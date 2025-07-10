# Reto Workshop Soroban Sesión 1 - Solución Avanzada

**Repositorio fork de John Ceciliano** con implementación de funciones adicionales para acumulación de sumas.

## Funcionalidades implementadas

1. **Función sumar(a, b) -> resultado**
   - Realiza la suma de dos números enteros
   - Almacena el resultado en el storage del contrato
   - Registra el resultado en el historial de operaciones

2. **Función resultado_anterior() -> último_resultado**
   - Devuelve el resultado de la última suma realizada
   - Retorna 0 si no hay operaciones previas

3. **Función extra: historial_sumas() -> Vec<i128>**
   - Devuelve un vector con todos los resultados históricos
   - Mantiene el orden cronológico (último resultado al final)

## Instrucciones para probar

1. Clonar el repositorio:
```bash
git clone https://github.com/tu-usuario/StarMakerSorobanV1RetoS1Mex-fork.git
cd StarMakerSorobanV1RetoS1Mex-fork
