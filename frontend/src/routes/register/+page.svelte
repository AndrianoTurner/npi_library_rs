<script>
	import { axiosClient } from '../../hooks.client';
	export let email = '';
	export let password = '';
	export let password2 = '';
	export let registerResponse = {
		status: '',
		errorCode: null
	};
	function register() {
		if (password != password2) {
			return;
		}
		let data = {
			email: email,
			password: password
		};
		axiosClient
			.post('user/register', JSON.stringify(data), {
				headers: {
					'Content-Type': 'application/json'
				}
			})
			.then(function (response) {
				console.log(response);
				registerResponse = {
					status: response.data.status,
					errorCode: response.data.errorCode
				};
			});
	}
</script>

<div class="centered">
	<h1>Регистрация</h1>

	<input bind:value={email} placeholder="email" type="email" />
	<p />
	<input bind:value={password} placeholder="password" type="password" />
	<p />
	<input bind:value={password2} placeholder="password" type="password" />
	<p />
	<button on:click={register}>Зарегистрироваться</button>
	<p />
	{#if registerResponse.errorCode}
		{registerResponse.status}
	{/if}

	{registerResponse.status}
	<p><a href="/register">Не зарегистрированы?</a></p>
</div>
