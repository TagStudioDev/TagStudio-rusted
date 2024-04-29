<script lang="ts">
	import Donut from '$components/Charts/Donut.svelte';

	let selected = '0';

	let fakeData = [
		{
			name: 'Curriculum_Vitae_2012.pdf',
			size: '3.2 MB',
			lastModified: '02/9/2012'
		},
		{
			name: 'party-invite.docx',
			size: '989 KB',
			lastModified: '02/9/2013'
		},
		{
			name: 'solar-system.pptx',
			size: '10.3 MB',
			lastModified: '24/12/2023'
		},
		{
			name: 'budget-estimate-Q1.xlsx',
			size: '2 MB',
			lastModified: '07/1/2024'
		},
		{
			name: 'poster.docx',
			size: '765 KB',
			lastModified: '02/9/2013'
		},
		{
			name: 'poster.docx',
			size: '765 KB',
			lastModified: '02/9/2013'
		},
		{
			name: 'poster.docx',
			size: '765 KB',
			lastModified: '02/9/2013'
		}
	];

	interface Programs {
		[name: string]: string;
	}

	let programs: Programs = {
		docx: 'Microsoft Word',
		pdf: 'Adobe Acrobat',
		pptx: 'Microsoft PowerPoint'
	};
</script>

<h1 class="text-4xl font-bold">My Files</h1>
<div class={`my-8 flex flex-grow bg-base-200 transition-all duration-500 ease-in-out`}>
	<!--Main Grid-->
	<div class="grid h-[25%] flex-grow grid-flow-col grid-cols-3 grid-rows-4 gap-8">
		<!--Nested Buttons Grid-->
		<div class="col-span-2 grid h-full w-full grid-cols-5 gap-8">
			<button
				class={`flex h-full w-full cursor-pointer flex-col items-start rounded-xl px-4 duration-300 ${selected === '0' ? 'bg-primary text-base-300' : 'bg-base-100'}`}
				on:click={() => (selected = '0')}
			>
				<svg
					width="48"
					height="48"
					viewBox="0 0 24 24"
					fill="none"
					class={`mt-4 duration-300 ${selected === '0' ? 'stroke-base-300' : ' stroke-red-700'}`}
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M19 7V13.8C19 14.9201 19 15.4802 18.782 15.908C18.5903 16.2843 18.2843 16.5903 17.908 16.782C17.4802 17 16.9201 17 15.8 17H12.2C11.0799 17 10.5198 17 10.092 16.782C9.71569 16.5903 9.40973 16.2843 9.21799 15.908C9 15.4802 9 14.9201 9 13.8V6.2C9 5.0799 9 4.51984 9.21799 4.09202C9.40973 3.71569 9.71569 3.40973 10.092 3.21799C10.5198 3 11.0799 3 12.2 3H15M19 7L15 3M19 7H16.6C16.0399 7 15.7599 7 15.546 6.89101C15.3578 6.79513 15.2049 6.64215 15.109 6.45399C15 6.24008 15 5.96005 15 5.4V3M5 7V14.6C5 16.8402 5 17.9603 5.43597 18.816C5.81947 19.5686 6.43139 20.1805 7.18404 20.564C8.03969 21 9.15979 21 11.4 21H15"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
				</svg>
				<p class="mt-4 text-sm">Total 4.2 GB</p>
				<h3 class="mt-2 text-xl font-bold">Documents</h3>
			</button>
			<button
				class={`flex h-full w-full cursor-pointer flex-col items-start rounded-xl px-4 duration-300 ${selected === '1' ? 'bg-primary text-base-300' : 'bg-base-100'}`}
				on:click={() => (selected = '1')}
			>
				<svg
					width="48"
					height="48"
					viewBox="0 0 24 24"
					class={`mt-4 duration-300 ${selected === '1' ? 'fill-base-300' : ' fill-green-700'}`}
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M12.5958 14.2834L11.8917 14.9934L12.5958 14.2834ZM14.6053 15.599L15.3094 16.309H15.3094L14.6053 15.599ZM18.275 19.9565L18.3957 20.9492L18.275 19.9565ZM16.2765 14.2543L15.9756 13.3006L16.2765 14.2543ZM19.1634 15.6514L18.4431 16.3451C18.4467 16.3488 18.4504 16.3525 18.454 16.3562L19.1634 15.6514ZM17.5236 14.2687L17.8463 13.3222L17.5236 14.2687ZM10.9266 12.9401L11.2278 11.9866H11.2278L10.9266 12.9401ZM8.04068 14.3326L8.7434 15.0441C8.74901 15.0386 8.75455 15.033 8.76002 15.0273L8.04068 14.3326ZM9.68048 12.9536L10.0023 13.9004H10.0023L9.68048 12.9536ZM5.09202 19.782L5.54601 18.891H5.54601L5.09202 19.782ZM4.21799 18.908L5.10899 18.454L5.10899 18.454L4.21799 18.908ZM19.782 18.908L18.891 18.454L19.782 18.908ZM18.908 19.782L18.454 18.891L18.908 19.782ZM5.09202 4.21799L5.54601 5.10899L5.54601 5.10899L5.09202 4.21799ZM4.21799 5.09202L5.10899 5.54601V5.54601L4.21799 5.09202ZM4.04409 18.2801L3.05159 18.4024L4.04409 18.2801ZM18.908 4.21799L18.454 5.10899L18.454 5.10899L18.908 4.21799ZM19.782 5.09202L18.891 5.54601V5.54601L19.782 5.09202ZM11.8917 14.9934L13.5597 16.6476L14.968 15.2274L13.3 13.5733L11.8917 14.9934ZM14.968 16.6476L15.3094 16.309L13.9011 14.8889L13.5597 15.2274L14.968 16.6476ZM13.5561 16.6439L17.5671 20.6629L18.9828 19.2501L14.9717 15.2311L13.5561 16.6439ZM15.3094 16.309C15.7242 15.8977 15.9928 15.6324 16.2133 15.447C16.425 15.269 16.5242 15.2247 16.5773 15.208L15.9756 13.3006C15.5663 13.4298 15.233 13.6582 14.9261 13.9163C14.6281 14.1669 14.2923 14.501 13.9011 14.8889L15.3094 16.309ZM19.8837 14.9577C19.5016 14.5609 19.1736 14.2192 18.8814 13.9618C18.5805 13.6967 18.2525 13.4606 17.8463 13.3222L17.2009 15.2152C17.2535 15.2331 17.3518 15.2797 17.5593 15.4625C17.7755 15.6529 18.0379 15.9243 18.4431 16.3451L19.8837 14.9577ZM16.5773 15.208C16.7806 15.1439 16.9991 15.1464 17.2009 15.2152L17.8463 13.3222C17.241 13.1158 16.5855 13.1082 15.9756 13.3006L16.5773 15.208ZM13.3 13.5733C12.9093 13.1859 12.5739 12.8522 12.2762 12.6018C11.9696 12.344 11.6366 12.1157 11.2278 11.9866L10.6253 13.8937C10.6784 13.9104 10.7775 13.9547 10.9889 14.1324C11.2091 14.3177 11.4774 14.5826 11.8917 14.9934L13.3 13.5733ZM8.76002 15.0273C9.16529 14.6076 9.42779 14.3369 9.64396 14.147C9.85143 13.9647 9.9496 13.9183 10.0023 13.9004L9.35868 12.0068C8.95276 12.1448 8.62476 12.3802 8.32385 12.6446C8.03164 12.9013 7.70353 13.2422 7.32134 13.638L8.76002 15.0273ZM11.2278 11.9866C10.6185 11.7941 9.96367 11.8012 9.35868 12.0068L10.0023 13.9004C10.2039 13.8319 10.4222 13.8295 10.6253 13.8937L11.2278 11.9866ZM16.8 19H7.2V21H16.8V19ZM5 16.8V7.2H3V16.8H5ZM7.2 19C6.62345 19 6.25117 18.9992 5.96784 18.9761C5.69617 18.9539 5.59545 18.9162 5.54601 18.891L4.63803 20.673C5.01641 20.8658 5.40963 20.9371 5.80497 20.9694C6.18864 21.0008 6.65645 21 7.2 21V19ZM5.54601 18.891C5.35785 18.7951 5.20487 18.6422 5.10899 18.454L3.32698 19.362C3.6146 19.9265 4.07354 20.3854 4.63803 20.673L5.54601 18.891ZM19 16.8C19 17.3766 18.9992 17.7488 18.9761 18.0322C18.9539 18.3038 18.9162 18.4045 18.891 18.454L20.673 19.362C20.8658 18.9836 20.9371 18.5904 20.9694 18.195C21.0008 17.8114 21 17.3436 21 16.8H19ZM16.8 21C17.4278 21 17.9636 21.0018 18.3957 20.9492L18.1542 18.9639C17.8715 18.9982 17.4809 19 16.8 19V21ZM18.3957 20.9492C18.7228 20.9094 19.0469 20.8336 19.362 20.673L18.454 18.891C18.4133 18.9117 18.3358 18.9418 18.1542 18.9639L18.3957 20.9492ZM18.891 18.454C18.7951 18.6422 18.6422 18.7951 18.454 18.891L19.362 20.673C19.9265 20.3854 20.3854 19.9265 20.673 19.362L18.891 18.454ZM7.2 3C6.65645 3 6.18864 2.99922 5.80497 3.03057C5.40963 3.06287 5.01641 3.13419 4.63803 3.32698L5.54601 5.10899C5.59545 5.0838 5.69617 5.04612 5.96784 5.02393C6.25117 5.00078 6.62345 5 7.2 5V3ZM5 7.2C5 6.62345 5.00078 6.25117 5.02393 5.96784C5.04612 5.69617 5.0838 5.59545 5.10899 5.54601L3.32698 4.63803C3.13419 5.01641 3.06287 5.40963 3.03057 5.80497C2.99922 6.18864 3 6.65645 3 7.2H5ZM4.63803 3.32698C4.07354 3.6146 3.6146 4.07354 3.32698 4.63803L5.10899 5.54601C5.20487 5.35785 5.35785 5.20487 5.54601 5.10899L4.63803 3.32698ZM3 16.8C3 17.4306 2.99819 17.9687 3.05159 18.4024L5.0366 18.1579C5.00181 17.8754 5 17.4844 5 16.8H3ZM3.05159 18.4024C3.0916 18.7273 3.16754 19.049 3.32698 19.362L5.10899 18.454C5.08839 18.4136 5.05863 18.3368 5.0366 18.1579L3.05159 18.4024ZM4.74682 18.9916L8.7434 15.0441L7.33796 13.6212L3.34137 17.5687L4.74682 18.9916ZM19 16.4934V16.8H21V16.4934H19ZM18.454 16.3562L19.2906 17.1982L20.7094 15.7887L19.8728 14.9466L18.454 16.3562ZM7.2 5H16.8V3H7.2V5ZM19 7.2V16.4934H21V7.2H19ZM16.8 5C17.3766 5 17.7488 5.00078 18.0322 5.02393C18.3038 5.04612 18.4045 5.0838 18.454 5.10899L19.362 3.32698C18.9836 3.13419 18.5904 3.06287 18.195 3.03057C17.8114 2.99922 17.3435 3 16.8 3V5ZM21 7.2C21 6.65645 21.0008 6.18864 20.9694 5.80497C20.9371 5.40963 20.8658 5.01641 20.673 4.63803L18.891 5.54601C18.9162 5.59545 18.9539 5.69617 18.9761 5.96784C18.9992 6.25117 19 6.62345 19 7.2H21ZM18.454 5.10899C18.6422 5.20487 18.7951 5.35785 18.891 5.54601L20.673 4.63803C20.3854 4.07354 19.9265 3.6146 19.362 3.32698L18.454 5.10899ZM16 8.99989C16 9.55218 15.5523 9.99989 15 9.99989V11.9999C16.6569 11.9999 18 10.6567 18 8.99989H16ZM15 9.99989C14.4477 9.99989 14 9.55218 14 8.99989H12C12 10.6567 13.3431 11.9999 15 11.9999V9.99989ZM14 8.99989C14 8.44761 14.4477 7.99989 15 7.99989V5.99989C13.3431 5.99989 12 7.34304 12 8.99989H14ZM15 7.99989C15.5523 7.99989 16 8.44761 16 8.99989H18C18 7.34304 16.6569 5.99989 15 5.99989V7.99989Z"
					/>
				</svg>

				<p class="mt-4 text-sm">Total 32.8 GB</p>
				<h3 class="mt-2 text-xl font-bold">Pictures</h3>
			</button>
			<button
				class={`flex h-full w-full cursor-pointer flex-col items-start rounded-xl px-4 duration-300 ${selected === '2' ? 'bg-primary text-base-300' : 'bg-base-100'}`}
				on:click={() => (selected = '2')}
			>
				<svg
					width="48"
					height="48"
					viewBox="0 0 24 24"
					class={`mt-4 duration-300 ${selected === '2' ? 'stroke-base-300' : ' stroke-blue-700'}`}
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M19 9V17.8C19 18.9201 19 19.4802 18.782 19.908C18.5903 20.2843 18.2843 20.5903 17.908 20.782C17.4802 21 16.9201 21 15.8 21H8.2C7.07989 21 6.51984 21 6.09202 20.782C5.71569 20.5903 5.40973 20.2843 5.21799 19.908C5 19.4802 5 18.9201 5 17.8V6.2C5 5.07989 5 4.51984 5.21799 4.09202C5.40973 3.71569 5.71569 3.40973 6.09202 3.21799C6.51984 3 7.0799 3 8.2 3H13M19 9L13 3M19 9H16.2C15.0799 9 14.5198 9 14.092 8.78201C13.7157 8.59027 13.4097 8.28431 13.218 7.90798C13 7.48016 13 6.9201 13 5.8V3M14.5 13C15.1137 13.4913 15.5 14.2053 15.5 15C15.5 15.7947 15.1137 16.5087 14.5 17M8.5 14H9.83333L11.5 12V18L9.83333 16H8.5V14Z"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
				</svg>

				<p class="mt-4 text-sm">Total 723 MB</p>
				<h3 class="mt-2 text-xl font-bold">Audio</h3>
			</button>
			<button
				class={`flex h-full w-full cursor-pointer flex-col items-start rounded-xl px-4 duration-300 ${selected === '3' ? 'bg-primary text-base-300' : 'bg-base-100'}`}
				on:click={() => (selected = '3')}
			>
				<svg
					width="48"
					height="48"
					viewBox="0 0 24 24"
					class={`mt-4 duration-300 ${selected === '3' ? 'stroke-base-300' : ' stroke-yellow-700'}`}
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M3 9H9.5M21 9H9.5M9.5 9L14.5 4M14.5 4H17.8C18.8467 4 19.4044 4 19.8221 4.1779M14.5 4H6.2C5.07989 4 4.51984 4 4.09202 4.21799C3.71569 4.40973 3.40973 4.71569 3.21799 5.09202C3 5.51984 3 6.07989 3 7.2V16.8C3 17.9201 3 18.4802 3.21799 18.908C3.40973 19.2843 3.71569 19.5903 4.09202 19.782C4.51984 20 5.07989 20 6.2 20H17.8C18.9201 20 19.4802 20 19.908 19.782C20.2843 19.5903 20.5903 19.2843 20.782 18.908C21 18.4802 21 17.9201 21 16.8V7.2C21 6.07989 21 5.51984 20.782 5.09202C20.5903 4.71569 20.2843 4.40973 19.908 4.21799C19.88 4.20371 19.8514 4.19037 19.8221 4.1779M9 4L4 9M15 9.00015L19.8221 4.1779M15 14.5L10 17.5V11.5L15 14.5Z"
						stroke-width="2"
						stroke-linejoin="round"
					/>
				</svg>

				<p class="mt-4 text-sm">Total 83.9 GB</p>
				<h3 class="mt-2 text-xl font-bold">Videos</h3>
			</button>
			<button
				class={`flex h-full w-full cursor-pointer flex-col items-start rounded-xl px-4 duration-300 ${selected === '4' ? 'bg-primary text-base-300' : 'bg-base-100'}`}
				on:click={() => (selected = '4')}
			>
				<svg
					width="48"
					height="48"
					viewBox="0 0 24 24"
					class={`mt-4 duration-300 ${selected === '4' ? 'fill-base-300' : ' fill-purple-700'}`}
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M13 3L13.7071 2.29289C13.5196 2.10536 13.2652 2 13 2V3ZM14 22C14.5523 22 15 21.5523 15 21C15 20.4477 14.5523 20 14 20V22ZM19 9H20C20 8.73478 19.8946 8.48043 19.7071 8.29289L19 9ZM18 10C18 10.5523 18.4477 11 19 11C19.5523 11 20 10.5523 20 10H18ZM5.21799 19.908L4.32698 20.362H4.32698L5.21799 19.908ZM6.09202 20.782L6.54601 19.891L6.54601 19.891L6.09202 20.782ZM6.09202 3.21799L5.63803 2.32698L5.63803 2.32698L6.09202 3.21799ZM5.21799 4.09202L4.32698 3.63803L4.32698 3.63803L5.21799 4.09202ZM13.109 8.45399L14 8V8L13.109 8.45399ZM13.546 8.89101L14 8L13.546 8.89101ZM9 16C8.44772 16 8 16.4477 8 17C8 17.5523 8.44772 18 9 18V16ZM12 18C12.5523 18 13 17.5523 13 17C13 16.4477 12.5523 16 12 16V18ZM9 12C8.44772 12 8 12.4477 8 13C8 13.5523 8.44772 14 9 14V12ZM13 14C13.5523 14 14 13.5523 14 13C14 12.4477 13.5523 12 13 12V14ZM9 8C8.44772 8 8 8.44772 8 9C8 9.55228 8.44772 10 9 10V8ZM10 10C10.5523 10 11 9.55228 11 9C11 8.44772 10.5523 8 10 8V10ZM17.2299 17.7929C16.8394 18.1834 16.8394 18.8166 17.2299 19.2071C17.6204 19.5976 18.2536 19.5976 18.6441 19.2071L17.2299 17.7929ZM15.0316 15.2507C14.8939 15.7856 15.2159 16.3308 15.7507 16.4684C16.2856 16.6061 16.8308 16.2841 16.9684 15.7493L15.0316 15.2507ZM17.9375 20C17.3852 20 16.9375 20.4477 16.9375 21C16.9375 21.5523 17.3852 22 17.9375 22V20ZM17.9475 22C18.4998 22 18.9475 21.5523 18.9475 21C18.9475 20.4477 18.4998 20 17.9475 20V22ZM13 2H8.2V4H13V2ZM4 6.2V17.8H6V6.2H4ZM8.2 22H14V20H8.2V22ZM19.7071 8.29289L13.7071 2.29289L12.2929 3.70711L18.2929 9.70711L19.7071 8.29289ZM20 10V9H18V10H20ZM4 17.8C4 18.3436 3.99922 18.8114 4.03057 19.195C4.06287 19.5904 4.13419 19.9836 4.32698 20.362L6.10899 19.454C6.0838 19.4045 6.04612 19.3038 6.02393 19.0322C6.00078 18.7488 6 18.3766 6 17.8H4ZM8.2 20C7.62345 20 7.25117 19.9992 6.96784 19.9761C6.69617 19.9539 6.59545 19.9162 6.54601 19.891L5.63803 21.673C6.01641 21.8658 6.40963 21.9371 6.80497 21.9694C7.18864 22.0008 7.65645 22 8.2 22V20ZM4.32698 20.362C4.6146 20.9265 5.07354 21.3854 5.63803 21.673L6.54601 19.891C6.35785 19.7951 6.20487 19.6422 6.10899 19.454L4.32698 20.362ZM8.2 2C7.65645 2 7.18864 1.99922 6.80497 2.03057C6.40963 2.06287 6.01641 2.13419 5.63803 2.32698L6.54601 4.10899C6.59545 4.0838 6.69617 4.04612 6.96784 4.02393C7.25117 4.00078 7.62345 4 8.2 4V2ZM6 6.2C6 5.62345 6.00078 5.25117 6.02393 4.96784C6.04612 4.69617 6.0838 4.59545 6.10899 4.54601L4.32698 3.63803C4.13419 4.01641 4.06287 4.40963 4.03057 4.80497C3.99922 5.18864 4 5.65645 4 6.2H6ZM5.63803 2.32698C5.07354 2.6146 4.6146 3.07354 4.32698 3.63803L6.10899 4.54601C6.20487 4.35785 6.35785 4.20487 6.54601 4.10899L5.63803 2.32698ZM12 3V7.4H14V3H12ZM14.6 10H19V8H14.6V10ZM12 7.4C12 7.66353 11.9992 7.92131 12.0169 8.13823C12.0356 8.36682 12.0797 8.63656 12.218 8.90798L14 8C14.0293 8.05751 14.0189 8.08028 14.0103 7.97537C14.0008 7.85878 14 7.69653 14 7.4H12ZM14.6 8C14.3035 8 14.1412 7.99922 14.0246 7.9897C13.9197 7.98113 13.9425 7.9707 14 8L13.092 9.78201C13.3634 9.92031 13.6332 9.96438 13.8618 9.98305C14.0787 10.0008 14.3365 10 14.6 10V8ZM12.218 8.90798C12.4097 9.2843 12.7157 9.59027 13.092 9.78201L14 8V8L12.218 8.90798ZM9 18H12V16H9V18ZM9 14H13V12H9V14ZM9 10H10V8H9V10ZM18.937 16C18.937 16.1732 18.8915 16.3053 18.6175 16.5697C18.4638 16.718 18.2828 16.8653 18.0319 17.074C17.7936 17.2723 17.5141 17.5087 17.2299 17.7929L18.6441 19.2071C18.86 18.9913 19.0805 18.8033 19.3109 18.6116C19.5287 18.4305 19.7852 18.2223 20.0065 18.0087C20.4825 17.5493 20.937 16.9314 20.937 16H18.937ZM17.937 15C18.4893 15 18.937 15.4477 18.937 16H20.937C20.937 14.3431 19.5938 13 17.937 13V15ZM16.9684 15.7493C17.0795 15.3177 17.4724 15 17.937 15V13C16.5377 13 15.3645 13.957 15.0316 15.2507L16.9684 15.7493ZM17.9375 22H17.9475V20H17.9375V22Z"
					/>
				</svg>

				<p class="mt-4 text-sm">Total 3.9 GB</p>
				<h3 class="mt-2 text-xl font-bold">Misc</h3>
			</button>
		</div>

		<div
			class="scrollbar-thumb-rounded-full scrollbar scrollbar-w-1 scrollbar-track-transparent scrollbar-thumb-primary col-span-2 row-span-2 mb-[-4rem] max-h-[100%] overflow-y-scroll rounded-xl bg-base-100"
		>
			<div class="overflow-x-auto">
				<table class="table max-h-fit">
					<!-- head -->
					<thead class="hidden">
						<tr>
							<th>
								<div class="form-control">
									<label class="label cursor-pointer">
										<input type="checkbox" class="checkbox-primary checkbox bg-base-100" />
									</label>
								</div>
							</th>
							<th>File Name</th>
							<th>File Type</th>
							<th>Last Modified</th>
							<th></th>
						</tr>
					</thead>

					<tbody>
						<!-- row 1 -->
						{#each fakeData as data}
							<tr>
								<th>
									<div class="form-control">
										<label class="label cursor-pointer">
											<input type="checkbox" class="checkbox-primary checkbox bg-base-100" />
										</label>
									</div>
								</th>
								<td>
									<div class="flex items-center gap-3">
										<div class="">
											<div class="h-9 w-9">
												{#if data.name.split('.')[1] === 'docx'}
													<img src="/icons/ms-word.svg" alt="Microsoft Word" />
												{:else if data.name.split('.')[1] === 'pdf'}
													<img src="/icons/adobe-acrobat.svg" alt="PDF" />
												{:else if data.name.split('.')[1] === 'pptx'}
													<img src="/icons/ms-ppt.svg" alt="Microsoft PowerPoint" />
												{:else if data.name.split('.')[1] === 'xlsx'}
													<img src="/icons/ms-excel.svg" alt="Microsoft Excel" />
												{/if}
											</div>
										</div>
										<div>
											<div class="font-bold">{data.name.split('.')[0]}</div>
											<div class="text-sm opacity-50">{programs[data.name.split('.')[1]]}</div>
										</div>
									</div>
								</td>
								<td class="uppercase">{data.name.split('.')[1]} </td>
								<td>{data.lastModified}</td>
								<th>
									<button class="btn-base-200 btn btn-xs">details</button>
								</th>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
		<div
			class="col-start-3 row-span-3 flex flex-col items-center justify-center rounded-xl bg-base-100"
		>
			<h2 class="m-6 self-start text-2xl font-bold">Storage</h2>
			<div class="h-[100%] w-[90%]">
				<Donut />
			</div>
		</div>
		<div
			class="scrollbar-thumb-rounded-full scrollbar scrollbar-w-1 scrollbar-track-transparent scrollbar-thumb-primary col-span-3 row-start-4 mb-[-18rem] max-h-[100%] overflow-y-scroll rounded-xl bg-base-100"
		>
			<div class="overflow-x-auto">
				<table class="table max-h-fit">
					<!-- head -->
					<thead class="hidden">
						<tr>
							<th>
								<div class="form-control">
									<label class="label cursor-pointer">
										<input type="checkbox" class="checkbox-primary checkbox bg-base-100" />
									</label>
								</div>
							</th>
							<th>File Name</th>
							<th>File Type</th>
							<th>Last Modified</th>
							<th></th>
						</tr>
					</thead>

					<tbody>
						<!-- row 1 -->
						{#each fakeData as data}
							<tr>
								<th>
									<div class="form-control">
										<label class="label cursor-pointer">
											<input type="checkbox" class="checkbox-primary checkbox bg-base-100" />
										</label>
									</div>
								</th>
								<td>
									<div class="flex items-center gap-3">
										<div class="">
											<div class="h-9 w-9">
												{#if data.name.split('.')[1] === 'docx'}
													<img src="/icons/ms-word.svg" alt="Microsoft Word" />
												{:else if data.name.split('.')[1] === 'pdf'}
													<img src="/icons/adobe-acrobat.svg" alt="PDF" />
												{:else if data.name.split('.')[1] === 'pptx'}
													<img src="/icons/ms-ppt.svg" alt="Microsoft PowerPoint" />
												{:else if data.name.split('.')[1] === 'xlsx'}
													<img src="/icons/ms-excel.svg" alt="Microsoft Excel" />
												{/if}
											</div>
										</div>
										<div>
											<div class="font-bold">{data.name.split('.')[0]}</div>
											<div class="text-sm opacity-50">{programs[data.name.split('.')[1]]}</div>
										</div>
									</div>
								</td>
								<td class="uppercase">{data.name.split('.')[1]} </td>
								<td>{data.lastModified}</td>
								<th>
									<button class="btn-base-200 btn btn-xs">details</button>
								</th>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	</div>
</div>
