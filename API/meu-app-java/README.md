# Meu App Java

Este é um projeto Java de exemplo que demonstra a estrutura básica de uma aplicação Java utilizando Maven.

## Estrutura do Projeto

```
meu-app-java
├── src
│   ├── main
│   │   ├── java
│   │   │   └── com
│   │   │       └── exemplo
│   │   │           └── App.java
│   │   └── resources
│   │       └── application.properties
├── pom.xml
└── README.md
```

## Instruções de Instalação

1. Clone este repositório.
2. Navegue até o diretório do projeto.
3. Execute o comando `mvn install` para compilar o projeto e baixar as dependências.

## Uso

Para executar a aplicação, utilize o comando:

```
mvn exec:java -Dexec.mainClass="com.exemplo.App"
```
