#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

// Função base que será decorada
typedef struct {
    char* (*get_message)(void);
} Message;

// Função concreta
char* simple_message() {
    return strdup("Alert: System Failure!");
}

// Decorador 1: Adiciona borda ao redor da mensagem
char* bordered_message(Message* base) {
    char* base_message = base->get_message();
    size_t len = strlen(base_message) + 10;
    char* bordered = (char*)malloc(len);
    snprintf(bordered, len, "| %s |", base_message);
    free(base_message);
    return bordered;
}

// Decorador 2: Converte a mensagem para caixa alta
char* uppercase_message(Message* base) {
    char* base_message = base->get_message();
    size_t len = strlen(base_message);
    char* uppercase = (char*)malloc(len + 1);
    for (size_t i = 0; i < len; i++) {
        uppercase[i] = toupper(base_message[i]);
    }
    uppercase[len] = '\0';
    free(base_message);
    return uppercase;
}

// Decorador 3: Adiciona um contador de alertas
char* counter_message(Message* base, int count) {
    char* base_message = base->get_message();
    size_t len = strlen(base_message) + 20;
    char* counted = (char*)malloc(len);
    snprintf(counted, len, "%s (Alert #%d)", base_message, count);
    free(base_message);
    return counted;
}

// Função de inicialização do Message
Message* create_message(char* (*decorator)(void)) {
    Message* msg = (Message*)malloc(sizeof(Message));
    msg->get_message = decorator;
    return msg;
}

// Limpa a memória de um Message
void free_message(Message* msg) {
    free(msg);
}

// Testando o padrão Decorador
int main() {
    // Mensagem base
    Message* simple = create_message(simple_message);

    // Decorando com borda
    Message bordered = { .get_message = (char* (*)(void))simple };
    bordered.get_message = (char* (*)(void))() { return bordered_message(simple); };

    printf("%s\n", bordered);

    return 0;
}
``
