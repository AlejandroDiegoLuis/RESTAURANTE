# 🍽️ Pedidos Restaurante en Solana

## 📌 Descripción

Este proyecto es un programa desarrollado en Solana utilizando Rust y Anchor que permite gestionar pedidos de un restaurante.

El sistema permite crear, actualizar, consultar y eliminar pedidos almacenados en la blockchain.

El desarrollo se realizó utilizando **Solana Playground (entorno web)**, lo que permite compilar, desplegar y probar contratos sin instalar herramientas locales.

---

## ⚙️ Funcionalidades (CRUD)

* 🟢 Crear pedido (CREATE)a
* 🟡 Actualizar cliente (UPDATE)
* 🔵 Ver pedido (READ)
* 🔴 Eliminar pedido (DELETE)

---

## 🧠 Uso de PDA (Program Derived Address)

El programa utiliza PDA para almacenar los pedidos de forma segura.

### Seeds utilizadas:

* `"pedido"`
* Wallet del usuario (`autoridad`)
* Nombre del platillo (`platillo`)

Esto garantiza:

* Que cada pedido sea único
* Que solo el creador pueda modificarlo
* Mayor seguridad en la gestión de datos

---

## 🌐 Entorno de desarrollo

El proyecto fue desarrollado completamente en:

* 🌍 Solana Playground

Desde ahí se realizaron:

* Compilación (`build`)
* Despliegue (`deploy`)
* Pruebas (`test`)

---

## 🛠️ Tecnologías utilizadas

* 🦀 Rust
* ⚓ Anchor Framework
* ☀️ Solana Blockchain

---

## 🚀 Cómo ejecutar el proyecto

### Opción 1: Solana Playground (recomendado)

1. Abrir Solana Playground
2. Cargar el proyecto
3. Ejecutar:

   * Build
   * Deploy
   * Test

---

### Opción 2: Local

```bash
git clone https://github.com/TU-USUARIO/pedidos-restaurante.git
cd pedidos-restaurante
anchor build
anchor deploy
```

---

## 🧪 Ejemplo de uso

### Crear pedido

* platillo: ESPAGUETI
* mesa: MESA5
* cliente: ALEJANDRO

### Actualizar pedido

* nuevoCliente: CARLOS

### Ver pedido

Muestra los datos almacenados en la cuenta PDA

### Eliminar pedido

Cierra la cuenta y devuelve los fondos al usuario

---

## 🔐 Seguridad

* Uso de PDA para evitar manipulaciones externas
* Validación de autoridad (`has_one = autoridad`)
* Control de acceso en operaciones sensibles

---

## 👤 Autor

Alejandro Diego Luis

---

## 📌 Notas finales

* El proyecto cumple con el uso de PDA requerido en Solana
* Se desarrolló completamente en entorno web (Solana Playground)
* Implementa correctamente el patrón CRUD en blockchain
