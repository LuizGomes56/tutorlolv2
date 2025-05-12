# TutorLoL v2

> 🔧 Projeto de reimplementação completa em **Rust**, com foco em performance, automação e concorrência.

As versões anteriores do TutorLoL, em JavaScript e TypeScript (privadas), estão desatualizadas:

- A versão **JavaScript**, a mais poderosa até então, está há **2+ anos sem updates**.
- A versão **TypeScript** (atualmente em produção) está parada há **9 meses**.

Versões públicas (protótipos, com resultados pouco confiáveis):

- [GORemakeTutorLoL](https://github.com/LuizGomes56/GORemakeTutorLoL)
- [RSRemakeTutorLoL](https://github.com/LuizGomes56/RSRemakeTutorLoL)

🎯 O objetivo desta versão é **superar ambas** — combinando a performance do Rust com automação e arquitetura moderna.

- Web Assembly (Wasm) com `Yew` + `wasm-bindgen`

---

## 🚀 Objetivos Principais

### 📦 Atualização automática

**Problema**: Atualizar os arquivos `.json` manualmente a cada patch (~2 semanas) é repetitivo e ineficiente.

**Soluções planejadas:**

- 🔁 Verificação automática da versão do jogo (diária)
- ⚡ Atualização reativa dos `.json` ao detectar mudanças
- 🧵 Uso de *green threads* (`tokio::spawn`) para paralelismo  
  ⏱️ Estimativa de ganho: de **1h15m (parcial)** ⟶ **2 minutos (completo)****

---

### ⚡ Velocidade

- JavaScript não suporta multithreading nativo. Cada requisição leva **~1 segundo**, mesmo com cache.
- Em Rust, com `tokio`, cada cálculo poderá rodar em paralelo, com overhead mínimo.

---

### 🧪 Aprendizado e Técnicas

- Otimização de alocação e uso de referências (`&T` vs `T`)
- Implementação de `traits`, `impl`, métodos e padrão idiomático Rust
- Evitar cópias desnecessárias (`clone`)
- Concorrência real com `tokio`, `Arc`, `Mutex`, `RwLock`
- Uso de `once_cell`, `lazy_static!`, `parking_lot`, etc.

---

## 📋 Requisitos do Aplicativo

- ✅ Calcular o **dano individual** de cada:
  - habilidade, item, runa, trait
- 🔄 Mostrar a **diferença antes/depois** de aplicar um item
- 📚 Permitir **acúmulo de dano** por fonte
- 💡 *Recursos opcionais*:

<table>
  <tr><td>🩸</td><td>Calcular <b>ganho de vida</b> por roubo de vida</td></tr>
  <tr><td>⏱️</td><td>Estimar <b>tempo de combate</b></td></tr>
  <tr><td>📊</td><td>Determinar <b>quem está ganhando</b></td></tr>
  <tr><td>💥</td><td>Calcular <b>DPS objetivo</b></td></tr>
  <tr><td>🛍️</td><td>Sugerir <b>melhor item</b> para o momento</td></tr>
  <tr><td>📈</td><td>Avaliar <b>valor percentual</b> de cada item</td></tr>
  <tr><td>🐉</td><td>Identificar impacto de um <b>dragão</b> no jogo</td></tr>
  <tr><td style="color:orange;">🆕</td><td><b>Estimar dano do inimigo</b> contra o jogador</td></tr>
  <tr><td style="color:orange;">🆕</td><td><b>Simular lutas</b> para prever o provável vencedor</td></tr>
</table>

---

## Status atual

- [x] Estrutura de projeto em Rust
- [ ] Monitoramento do meta de forma automatizada
- [x] Sistema de cache automatizado
- [ ] Calculo básico de itens, habilidades e runas
- [ ] Avaliação de itens e builds
- [ ] WASM

---