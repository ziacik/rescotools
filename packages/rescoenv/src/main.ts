import { AzureEnvRetriever } from './app/azure-env-retriever';
import { env } from 'process';

const token = env.AZURE_PERSONAL_ACCESS_TOKEN;

if (!token) {
	throw new Error('Please set AZURE_PERSONAL_ACCESS_TOKEN env var.');
}

const retriever = new AzureEnvRetriever('https://dev.azure.com/resconet', token);

retriever.retrieveFor(19502).then(console.log).catch(console.error);
