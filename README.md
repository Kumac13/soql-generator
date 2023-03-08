# soql-generator

soql-generator is a tool for interactively executing SOQL (Salesforce Object Query Language) queries.

## Installation
You can install soql-generator using Cargo, the Rust package manager. First, clone the repository:

```bash
git clone https://github.com/your-username/soql-generator.git
```


Then, navigate to the cloned directory and run the following command:

```bash
cargo install --path .
```

## Environment Variables

Before using soql-generator, you need to set the following environment variables:

- SFDC_CLIENT_ID: Consumer key of your connected app
- SFDC_CLIENT_SECRET: Consumer secret of your connected app
- SFDC_USERNAME: Username of the Salesforce account you want to query
- SFDC_USERPASSWORD: Password of the Salesforce account you want to query

## Usage
Once you have installed soql-generator and set the required environment variables, you can use it to interactively execute SOQL queries. For example, you can execute a query like Account.where(Name = 'Test') to retrieve all accounts with the name "Test".

To start soql-generator, simply run the following command:

```bash
soql-generator
```

This will launch an interactive prompt where you can enter your SOQL queries and see the results immediately.



