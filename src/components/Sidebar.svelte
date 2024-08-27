<script>
    let { children } = $props();
</script>

<div class="flex h-[calc(100%)] w-full flex-wrap">
    <!-- Sidebar -->
    <div class="w-[15vw] rounded-lg bg-base-200">
        <div class="mt-8">
            <input type="text" placeholder="Search a Tag" class="input input-bordered ml-4 h-10 w-[90%]" />
            <ul class="menu menu-md w-full max-w-xs rounded-lg bg-base-200 font-semibold">
                <li>
                    <button>
                        <div class="h-[0.6rem] w-[0.6rem] rounded-full bg-yellow-400"></div>
                        School
                    </button>
                </li>
                <li>
                    <details open>
                        <summary>
                            <div class="h-[0.6rem] w-[0.6rem] rounded-full bg-blue-700"></div>
                            Photos
                        </summary>
                        <ul>
                            <li>
                                <button>
                                    <div class="h-[0.6rem] w-[0.6rem] rounded-full bg-teal-400"></div>
                                    Cat
                                </button>
                            </li>
                            <li>
                                <button>
                                    <div class="h-[0.6rem] w-[0.6rem] rounded-full bg-amber-950"></div>
                                    Dog
                                </button>
                            </li>
                            <li>
                                <details open>
                                    <summary>
                                        <div class="h-[0.6rem] w-[0.6rem] rounded-full bg-red-600"></div>
                                        Nintendo
                                    </summary>
                                    <ul>
                                        <li>
                                            <button>
                                                <div class="h-[0.6rem] w-[0.6rem] rounded-full bg-green-800"></div>
                                                Luigi's Mansion
                                            </button>
                                        </li>
                                        <li>
                                            <button>
                                                <div class="h-[0.6rem] w-[0.6rem] rounded-full bg-purple-700"></div>
                                                Mario Kart
                                            </button>
                                        </li>
                                        <li>
                                            <details open>
                                                <summary>
                                                    <div class="h-[0.6rem] w-[0.6rem] rounded-full bg-black"></div>
                                                    Console
                                                </summary>
                                                <ul>
                                                    <li>
                                                        <button>
                                                            <div
                                                                class="h-[0.6rem] w-[0.6rem] rounded-full bg-stone-300"
                                                            ></div>
                                                            Wii
                                                        </button>
                                                    </li>
                                                </ul>
                                            </details>
                                        </li>
                                    </ul>
                                </details>
                            </li>
                        </ul>
                    </details>
                </li>
                <li>
                    <button>
                        <div class="h-[0.6rem] w-[0.6rem] rounded-full bg-sky-600"></div>
                        Programming
                    </button>
                </li>
            </ul>
        </div>
        <ul class="menu menu-md w-[100%] rounded-box bg-base-200">
            <li>
                <h2 class="menu-title text-xl text-gray-300">Vaults</h2>
                <ul>
                    <li>
                        <button class="bg-base-100"
                            >Personal
                            <div class="badge badge-md rounded-full bg-red-700">3</div></button
                        >
                    </li>
                    <li><button>Work</button></li>
                    <li><button>Top Secret</button></li>
                </ul>
            </li>
        </ul>
    </div>
    <!-- Main content -->
    <div class="flex-1">
        {@render children()}
    </div>
</div>
