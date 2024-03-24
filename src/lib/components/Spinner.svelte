<div class="loader"></div>

<style scoped>
	.loader {
		animation: l5 2s infinite;
		aspect-ratio: 1;
		background: radial-gradient(farthest-side, #ed303c 94%, #0000),
			radial-gradient(farthest-side, #3b8183 94%, #0000),
			radial-gradient(farthest-side, #fad089 94%, #0000),
			radial-gradient(farthest-side, #ff9c5b 94%, #0000), #ed303c;
		background-repeat: no-repeat;
		background-size: 105% 105%;
		border-radius: 50%;
		width: 40px;
	}

	@keyframes l5 {
		0% {
			background-position:
				50% -50px,
				-40px 50%,
				50% calc(100% + 50px),
				calc(100% + 50px) 50%;
		}
		20%,
		25% {
			background-position:
				50% -50px,
				-50px 50%,
				50% calc(100% + 50px),
				50% 50%;
		}
		45%,
		50% {
			background-position:
				50% -50px,
				-50px 50%,
				50% 50%,
				50% 50%;
		}
		75% {
			background-position:
				50% -50px,
				50% 50%,
				50% 50%,
				50% 50%;
		}
		95%,
		100% {
			background-position:
				50% 50%,
				50% 50%,
				50% 50%,
				50% 50%;
		}
	}
</style>
