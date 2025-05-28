# TutorLoL v2

> 🔧 Projeto de reimplementação completa em **Rust**, com foco em performance, automação e concorrência.

<span style="color:#f1fa8c;">Este é apenas a parte do SERVIDOR do projeto completo. Verifique o [Client-Side](https://github.com/LuizGomes56/tlv2app), que utiliza <i style="color:#caf2fa;">Tauri (Windows) e Yew (WebAssembly) combinados.</i></span>

As versões anteriores do TutorLoL, em JavaScript e TypeScript (privadas), estão desatualizadas:

- A versão **JavaScript**, a mais poderosa até então, está há **2+ anos sem updates**.
- A versão **TypeScript** (atualmente em produção) está parada há **9 meses**.

Versões públicas (protótipos, com resultados pouco confiáveis):

- [GORemakeTutorLoL](https://github.com/LuizGomes56/GORemakeTutorLoL)
- [RSRemakeTutorLoL](https://github.com/LuizGomes56/RSRemakeTutorLoL)

🎯 O objetivo desta versão é **superar ambas** — combinando a performance do Rust com automação e arquitetura moderna.

- Web Assembly (Wasm) com `Yew` + `wasm-bindgen`. Tauri (Desktop)

---

## 🚀 Objetivos Principais

### 📦 Atualização automática

**Problema**: Atualizar os arquivos `.json` manualmente a cada patch (~2 semanas) é repetitivo e ineficiente.

**Soluções planejadas:**

- 🔁 Verificação automática da versão do jogo (diária)
- ⚡ Atualização reativa dos `.json` ao detectar mudanças
- 🧵 Uso de *green threads* (`tokio::spawn`) para paralelismo  
  ⏱️ Estimativa de ganho: de **1h15m (parcial)** ⟶ **5 minutos (completo)**

---

### ⚡ Velocidade

- JavaScript não suporta multithreading nativo. Cada requisição leva **~1 segundo**, mesmo com cache.
- Em Rust, com `tokio`, cada cálculo poderá rodar em paralelo, com overhead mínimo.
- **Estimativa**: **96 vezes** mais rápido que o JavaScript.
- <b style="color:rgb(255, 170, 182)">RUST:</b> **~210 microssegundos por cálculo**. 
- <b style="color:rgb(255, 170, 182)">JS:</b> **~19 millissegundos por cálculo**

## Benchmark de resposta do servidor

### 1 milhão de respostas síncronas

**TypeScript**: 1202 segundos (832 req/s)


**Rust**: 775 segundos (1290 req/s)

- 1.55x de diferença de velocidade no momento


### Async (64 goroutines), 1024 requests

**TypeScript**: 1.21 segundos (846 req/s)


**Rust**: 0.21 segundos (4947 req/s)

### Uso de recursos

**TypeScript**: (11% - 21%) CPU, (465MB - 600MB) RAM


**Rust**: (11% - 15%) CPU, (4MB - 6MB) RAM

- Rust usou aproximadamente 1% da memória usada pelo TypeScript
- Uso de CPU similar, porém mais estável

### Falhas usando 512 goroutines, 1 << 16 requests

**TypeScript**: 92.6% de falha


**Rust**: 0.9% de falha

- As requisições para o servidor Rust-Actix falharam apenas no começo. 
Após as primeiras 512 requisições, o servidor Rust terminou as tarefas significativamente mais devagar,
com média de 200 req/s, demonstrando a degradação do servidor, mas ainda sendo capaz de enviar respostas.
- O TypeScript não foi capaz de gerenciar mais que 100 requisições ao mesmo tempo, virtualmente ignorando
todas que estiverem fora de suas capacidades

---

## Status atual

- [X] Atualização e setup automáticos
- [ ] Definição da estrutura do JSON para cada personagem (3 de 172)
- [ ] Definição para atualização automática dos itens (0 de 315)
- [ ] Tratamento de itens e campeões que são excessão
- [ ] Construção de funcionalidades novas (0 de 5)
- [ ] Configurar SEO
- [ ] Configurar o servidor para manter o projeto rodando
- [ ] Proteção contra panics (Não iniciado)
- [x] Estrutura de projeto em Rust
- [x] Monitoramento do meta de forma automatizada
- [x] Sistema de cache automatizado
- [ ] Otimização e simplificaçao do código
- [x] Calculo básico de itens, habilidades e runas
- [ ] Avaliação de itens e builds
- [X] WASM (Setup frontend)
- [-] Sobreposição no jogo (overlay)
- [X] Todas as features disponíveis no projeto TypeScript concluídas
- [ ] Cálculos confiáveis
- [ ] Benchmark de diferença de performance entre todas as aplicações
- [ ] Integração do frontend com o App Windows (Tauri)

---