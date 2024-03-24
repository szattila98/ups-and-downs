<script lang="ts">
	import {
		computePosition,
		type Placement,
		flip,
		shift,
		offset as offsetMw,
		arrow
	} from '@floating-ui/dom';
	import type { Action } from 'svelte/action';

	export let placement: Placement = 'top';
	export let show: 'click' | 'hover' = 'hover';
	export let offset: number = 5;

	let hostElement: HTMLDivElement | null = null;
	let tooltipElement: HTMLDivElement | null = null;
	let arrowElement: HTMLDivElement | null = null;

	const update = () => {
		if (!hostElement || !tooltipElement || !arrowElement) return;
		computePosition(hostElement, tooltipElement, {
			placement,
			middleware: [
				offsetMw(offset),
				flip(),
				shift({ padding: 5 }),
				arrow({ element: arrowElement })
			]
		}).then(({ x, y, placement, middlewareData }) => {
			if (!tooltipElement || !arrowElement) return;
			Object.assign(tooltipElement.style, {
				left: `${x}px`,
				top: `${y}px`
			});

			if (!middlewareData.arrow) return;
			const { x: arrowX, y: arrowY } = middlewareData.arrow;

			const staticSide = (
				{
					top: 'bottom',
					right: 'left',
					bottom: 'top',
					left: 'right'
				} as const
			)[placement.split('-')[0]];

			if (!staticSide) return;
			Object.assign(arrowElement.style, {
				left: arrowX != null ? `${arrowX}px` : '',
				top: arrowY != null ? `${arrowY}px` : '',
				right: '',
				bottom: '',
				[staticSide]: '-4px'
			});
		});
	};

	const showTooltip = () => {
		if (!tooltipElement) return;
		tooltipElement.style.display = 'block';
		update();
	};

	const hideTooltip = () => {
		if (!tooltipElement) return;
		tooltipElement.style.display = '';
	};

	$: shown = tooltipElement?.style.display === 'block';

	const handlers: Action<HTMLDivElement> = (element) => {
		if (show === 'click') {
			const toggle = () => (shown ? hideTooltip() : showTooltip());
			element.addEventListener('click', toggle);
			element.addEventListener('keydown', (event) => {
				if (event.key === 'Enter') {
					toggle();
				} else if (event.key === 'Escape' || event.key === 'Tab') {
					hideTooltip();
				}
			});
		}
		if (show === 'hover') {
			element.addEventListener('mouseenter', showTooltip);
			element.addEventListener('mouseleave', hideTooltip);
			element.addEventListener('focus', showTooltip);
			element.addEventListener('blur', hideTooltip);
		}
	};
</script>

<div
	id="host"
	bind:this={hostElement}
	use:handlers
	aria-describedby="tooltip"
	role="button"
	tabindex="0"
	class:clickable={show === 'click'}
>
	<slot name="host">
		<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"
			><path
				d="M11 10.9794C11 10.4271 11.4477 9.97937 12 9.97937C12.5523 9.97937 13 10.4271 13 10.9794V16.9794C13 17.5317 12.5523 17.9794 12 17.9794C11.4477 17.9794 11 17.5317 11 16.9794V10.9794Z"
				fill="currentColor"
			/><path
				d="M12 6.05115C11.4477 6.05115 11 6.49886 11 7.05115C11 7.60343 11.4477 8.05115 12 8.05115C12.5523 8.05115 13 7.60343 13 7.05115C13 6.49886 12.5523 6.05115 12 6.05115Z"
				fill="currentColor"
			/><path
				fill-rule="evenodd"
				clip-rule="evenodd"
				d="M12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2ZM4 12C4 16.4183 7.58172 20 12 20C16.4183 20 20 16.4183 20 12C20 7.58172 16.4183 4 12 4C7.58172 4 4 7.58172 4 12Z"
				fill="currentColor"
			/></svg
		>
	</slot>
</div>
<div bind:this={tooltipElement} id="tooltip" role="tooltip">
	<slot name="content" />
	<div bind:this={arrowElement} id="arrow"></div>
</div>

<style scoped>
	#host {
		height: fit-content;
		width: fit-content;
	}

	#tooltip {
		background: var(--highlight);
		border-radius: 4px;
		color: var(--light);
		display: none;
		font-size: 90%;
		font-weight: bold;
		left: 0;
		padding: 5px;
		position: absolute;
		top: 0;
		width: max-content;
	}

	#arrow {
		background: var(--highlight);
		height: 8px;
		position: absolute;
		transform: rotate(45deg);
		width: 8px;
	}

	svg {
		height: var(--default-icon-size, 28px);
		width: var(--default-icon-size, 28px);
	}

	.clickable {
		cursor: pointer;
	}
</style>
