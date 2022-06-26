import { AzureEnvRetriever } from './app/azure-env-retriever';
import { env } from 'process';
import { program } from 'commander';

const token = env.AZURE_PERSONAL_ACCESS_TOKEN;

if (!token) {
	throw new Error('Please set AZURE_PERSONAL_ACCESS_TOKEN env var.');
}

program.argument('<workitem-id>', 'work item id').parse();

program.action((workitemIdStr: string) => {
	console.log('Retrieving', workitemIdStr);
	const retriever = new AzureEnvRetriever('https://dev.azure.com/resconet', token);
	retriever
		.retrieveFor(+workitemIdStr)
		.then(console.log)
		.catch(console.error);
});

program.parse();
