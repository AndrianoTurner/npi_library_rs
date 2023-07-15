<script>
	import { onMount, onDestroy } from 'svelte';
	export let book;
	let docEditor;
	let config = {
		height: '1080px',
		width: '1920px',
		document: {
			fileType: book.filetype,
			key: book.key,
			title: book.title,
			url: book.downloadUrl
		},
		documentType: book.documentType,
		editorConfig: {
			callbackUrl: book.callbackUrl
		}
	};

	console.log(config);

	onMount(() => {
		if (!document) {
			return;
		}
		let script = document.createElement('script');
		script.src = 'http://localhost:8000/web-apps/apps/api/documents/api.js';
		script.type = 'text/javascript';
		document.head.append(script);
		script.onload = function () {
			docEditor = new DocsAPI.DocEditor('placeholder1', config);
		};
	});
	onDestroy(() => {
		if (!document) {
			return;
		}
		let scripts = document.getElementsByTagName('script');
		for (var script of scripts) {
			console.log(script);
			if (
				script.src == 'http://localhost:8000/web-apps/apps/api/documents/api.js' &&
				script.type == 'text/javascript'
			) {
				console.log(script);
				script.remove();
				docEditor.destroyEditor();
			}
		}
	});
</script>

<div id="placeholder1" />
