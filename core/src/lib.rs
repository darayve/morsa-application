//! Núcleo do Morsa.
//!
//! Recebe bytes e opções, devolve bytes e um relatório. Não conhece telas
//! nem rede. Cada módulo abaixo tem uma responsabilidade:
//!
//! - `api`: a fronteira com o Flutter (única parte que conhece a ponte);
//! - `detect`: formato real pelo conteúdo;
//! - `error`: erros tipados com os códigos `E-xxx`;
//! - `image`: conversão de imagens;
//! - `limits`: limites de segurança.

pub mod api;
pub mod detect;
pub mod error;
pub mod image;
pub mod limits;

// Código gerado pelo flutter_rust_bridge (não editar à mão).
#[rustfmt::skip]
mod frb_generated;
