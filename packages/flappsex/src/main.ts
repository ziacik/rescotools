import fetch from 'cross-fetch';
import { parse } from 'set-cookie-parser';

async function run(): Promise<void> {
	const result = await fetch('https://resco.flapps.com/', {
		credentials: 'omit',
		headers: {},
		method: 'GET',
		mode: 'cors',
	});
	const cookieHeader = result.headers.get('set-cookie') ?? '';
	const cookies = parse(cookieHeader, { map: true });
	const cookie = cookies['JSESSIONID'].value;

	const l = await fetch('https://resco.flapps.com/login.jsp', {
		credentials: 'include',
		headers: {
			'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:105.0) Gecko/20100101 Firefox/105.0',
			Accept: 'text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8',
			'Accept-Language': 'en-US,en;q=0.5',
			'Upgrade-Insecure-Requests': '1',
			Cookie: `JSESSIONID=${cookie}`,
			'Sec-Fetch-Dest': 'document',
			'Sec-Fetch-Mode': 'navigate',
			'Sec-Fetch-Site': 'none',
			'Sec-Fetch-User': '?1',
		},
		method: 'GET',
		mode: 'cors',
	});

	const body = await l.text();
	const csrfI = body.indexOf(`_csrf" value="`) + `_csrf" value="`.length;
	const csrf = body.substring(csrfI, csrfI + '8dc1ace5-29e4-49ac-a857-07e95e756c3c'.length);
	console.log(csrf);

	const login = await fetch('https://resco.flapps.com/login_check', {
		credentials: 'include',
		headers: {
			Accept: 'text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8',
			'Content-Type': 'application/x-www-form-urlencoded',
			Cookie: `JSESSIONID=${cookie};user=frantisek.ziacik`,
		},
		referrer: 'https://resco.flapps.com/login.jsp',
		body: `_csrf=${csrf}&backUrl=&instance=resco&login=frantisek.ziacik&password=kvakykvak100%3F`,
		method: 'POST',
		mode: 'cors',
		redirect: 'manual',
	});

	console.log('XX', login.headers, 'YY');

	const newCookieHeader = login.headers.get('set-cookie') ?? '';
	const newCookies = parse(newCookieHeader, { map: true });
	const newCookie = newCookies['JSESSIONID'].value;

	console.log(cookie, newCookie);

	const absences = await fetch('https://resco.flapps.com/rest/dashboard/absences', {
		credentials: 'include',
		headers: {
			Accept: 'application/json, text/plain, */*',
			Cookie: `JSESSIONID=${newCookie}`,
		},
		method: 'GET',
		mode: 'cors',
	});

	const xxx = await absences.json();
	const b = xxx.map((it: Record<string, unknown>) => `${it.fullName}: ${it.type}`);
	console.log(b);
}

run().then(() => console.log('Ok.'));
