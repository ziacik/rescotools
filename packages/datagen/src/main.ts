import { program } from 'commander';
import { getName } from 'fake-names';

program.argument('[count]', 'number of items to generate', 10).parse();

program.action((countStr: string) => {
	const count = +countStr;
	writeHeader();
	writeBody(count);
});

program.parse();

function writeHeader(): void {
	console.log('Name;City;Country;Currency;E-mail;Main Phone;Postal Code;State;Street 1;Street 2');
}

function writeBody(count: number): void {
	for (let i = 0; i < count; i++) {
		writeLine();
	}
}

function writeLine(): void {
	const data = [getName(), getName(), getName(), 'Euro', 'someone@example.com', '111-2222', '22233', 'MA', getName(), '123'];
	console.log(data.join(';'));
}
