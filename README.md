# TutorLoL v2

A versão inicial feita em JavaScript, também a mais poderosa, já está a mais de 2 anos sem nenhum tipo de atualização, enquanto que a versão atual em produção, feita em TypeScript não é atualizada a 9 meses. Então, esse projeto está destinado a se tornar tão poderosa quanto a versão em JavaScript, e com mais algumas features.

## Objetivos

### Update automático

O principal problema referente às versões já construídas é o fato de elas serem incapazes de se atualizar por completo de forma automática. No momento, é necessário ir manualmente em cada arquivo `.json`, verificando as atualizações que foram feitas nos últimos patchs, o que é um trabalho exaustivo e repetitivo, pois o jogo se atualiza a cada 2 semanas.

<b>Solução</b>
- Criar um WebScrapper para a Wiki oficial do jogo e atualizar de forma automática todos os arquivos `.json`.
- Verificação diária automática da versão do jogo, e quando uma mudança for detectada, o APP se atualizar automaticamente.
- Uso de green threads para acelerar o tempo de atualização dos arquivos `.json`. Estimado reduzir de 1h15m (parcial) para 2 minutos (Completo)

### Velocidade

As versões atuais se restringem a um tempo mínimo de 1s para cada requisição pois JavaScript não suporta multiplos threads, o que faz com que o app fique lento caso haja várias requisições simultâneas.

### Aprendizado

- Compreender melhor uso de memória e referências (Evitar cópias desnecessárias)
- Uso restrito de bibliotecas. As calculadoras não podem usar recursos externos. Requisições HTTP usarão AWC. RegEx não é permitido.
- Implementar traits e métodos aos structs
- Implementar concorrência para resolver cálculos em paralelo
- lazy_static!, once_cell, mutexes, Arc, RwLock, e estruturas thread-safe para leitura.

## Requerimentos do app

- Calcular o dano separado de cada habilidade, item, runa, trait, para cada personagem
- Mostrar a diferença com a adição de um item
- Permitir acumular o resultado de cada dano de forma individual
- <span style="color:rgb(148, 237, 250)">[Opcional]</span> Incluir ganho de vida com base no roubo de vida
- <span style="color:rgb(148, 237, 250)">[Opcional]</span> Estimar tempo de combate
- <span style="color:rgb(148, 237, 250)">[Opcional]</span> Avaliar qual jogador está na frente.
- <span style="color:rgb(148, 237, 250)">[Opcional]</span> Calcular DPS de forma objetiva
- <span style="color:rgb(148, 237, 250)">[Opcional]</span> Estimar qual o melhor item para se comprar com base no momento
- <span style="color:rgb(148, 237, 250)">[Opcional]</span> Avaliar o valor de cada item em porcentagem, baseado no contexto atual
- <span style="color:rgb(148, 237, 250)">[Opcional]</span> Identificar a diferença que a adição de um dragão para o outro time causaria
- <span style="color:rgb(250, 201, 148)">[NEW]</span> Tentar estimar o dano que o inimigo causaria ao próprio jogador
- <span style="color:rgb(250, 201, 148)">[NEW]</span> Estimar quem provavelmente venceria, qual time está na ganhando/perdendo