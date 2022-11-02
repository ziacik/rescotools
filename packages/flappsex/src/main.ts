import fetch from 'cross-fetch';
import { parse } from 'set-cookie-parser';

async function run(): Promise<void> {
	const login = await fetch('https://resco.flapps.com/login_check', {
		headers: {
			'Content-Type': 'application/x-www-form-urlencoded',
		},
		body: `instance=resco&login=frantisek.ziacik&password=heslo`,
		method: 'POST',
		redirect: 'manual',
	});

	const cookieHeader = login.headers.get('set-cookie') ?? '';
	const cookies = parse(cookieHeader, { map: true });
	const cookie = cookies['JSESSIONID'].value;

	const absences = await fetch('https://resco.flapps.com/rest/dashboard/absences', {
		headers: {
			Accept: 'application/json, text/plain, */*',
			Cookie: `JSESSIONID=${cookie}`,
		},
		method: 'GET',
		mode: 'cors',
	});

	const xxx = await absences.json();
	const b = xxx.map((it: Record<string, unknown>) => `${it.fullName}: ${it.type}`);
	console.log(b);
}

run().then(() => console.log('Ok.'));
