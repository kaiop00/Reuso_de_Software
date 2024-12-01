Padrão Decorador
O padrão Decorador resolve este problema ao permitir que comportamentos sejam adicionados dinamicamente a objetos. Ele encapsula o objeto base e aplica incrementos de funcionalidade, mantendo a interface original. Isso elimina a necessidade de modificar a classe original ou criar múltiplas subclasses.


Problema Fictício
Um sistema de alertas deve gerar mensagens com diferentes características. Por padrão, as mensagens são simples, mas os comportamentos adicionais podem incluir:

-Adicionar uma borda ao redor da mensagem.
-Colocar a mensagem em caixa alta.
-Adicionar um contador de alertas ao final.



Consequência
Vantagens:
-Flexibilidade: Comportamentos podem ser combinados de forma dinâmica, sem modificar a estrutura do código existente.
-Responsabilidade única: Cada decorador implementa um comportamento específico, seguindo o princípio da responsabilidade única.
-Reutilização: Decoradores podem ser reutilizados em diferentes combinações, reduzindo duplicação de código.

Desvantagens:
-Complexidade: A implementação com vários decoradores pode dificultar o rastreamento e a depuração de como os comportamentos são aplicados.
-Sobrecarga de objetos: O uso excessivo de decoradores pode levar a uma cadeia longa de objetos encadeados, afetando o desempenho e a legibilidade.