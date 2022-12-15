<main class="container">
    <div class="main">
        <h1 class="browse">Browse</h1>

        <form class="search" action="/">
            <input type="text">
            <button type="submit">
                <i class="fa-solid fa-magnifying-glass"></i>
            </button>
        </form>
    </div>
</main>

<style>
    .browse {
        text-align: left;
    }

    .search {
        display: flex;
    }

    .search > input {
        color: white;
        background-color: rgb(41, 41, 41);
        border-color: #393939;
        border-style: solid;
        border-width: 4px;
        border-radius: 3px;
        padding: 5px 5px 5px 5px;

        flex-grow: 1;
    }

    .search > input[type=text]:focus {
        outline-style: none;
    }

    .search > button {
        margin-left: 10px;
        padding: 5px 10px 5px 10px;
        background-color: rgb(41, 41, 41);
        border-color: #393939;
        border-style: solid;
        border-width: 4px;
        border-radius: 3px;

        cursor: pointer;
    }

    .search > button > i {
        color: white;
    }
</style>