<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  //Initialising the array which will hold the sudoku values
  let solution: number[] = $state(new Array);
  let puzzle: number[] = $state(new Array);

  //List of possible difficulties. Stored as strings for now
  let difficulties: string[] = $state(["easy", "medium", "hard"]);
  let isSolved: boolean = $state(false);

  //Function to generate puzzle of a given difficult.
  async function set_puzzle_and_reset(difficulty: string) : Promise<void> {
    puzzle = await invoke ("gen_random_sudoku", { difficulty: difficulty });
    solution = await invoke ("solve", { sudoku : puzzle });
    isSolved = false;
  }

  //Default starting difficulty
  let selected_difficulty = $state("easy");

  //Generating the default puzzle
  set_puzzle_and_reset("easy");

  function arrays_are_equal<T>(a1: T[], a2: T[]) : boolean
  //Enforcing at type system level that a1 and a2 are arrays of the same type T
  {
    if (a1.length !== a2.length){
      // Arrays of different length cannot be equal
      return false

    } else {
      // No bounds checking required as a1 and a2 are known to have the same length.
      // Otherwise in a2[index], index could be out of bounds
      //
      // Using === comparison to prevent type coersion
      // Not strictly necessary as the elements of the array definitely have the same type
      return a1.every((value, index) => a2[index] === value)
    }
  }

  function update_board(value: string, index: number) : void
  {
    let parsed_value: number = Number.parseInt(value);
    puzzle[index] = parsed_value;
    if (arrays_are_equal(puzzle, solution)){
      isSolved = true;
    }
  }
  
  async function check_is_solved() : Promise<void>
  {
    isSolved = await invoke("is_solved", { sudoku : puzzle });
  }

  
  async function reveal_solution() : Promise<void>
  {
    puzzle = solution;
    isSolved = true;
  }

</script>

<main class="container">

  <div class="board_container">
    {#key puzzle}
      <div class="board">
        {#each puzzle as square_value, i}
          {#if puzzle[i] == solution[i]}
            <div class="board_item">{square_value}</div>
          {:else}
            {let value = square_value == 0 ? "" : square_value.toString()}
            <div class="board_item">
              <input class="board_item_input" type="text" maxlength="1" bind:value={value} oninput={() => update_board(value, i)}>
            </div>
          {/if}
        {/each}
      </div><br>
    {/key}

    <div>
      <select bind:value={selected_difficulty}>
        {#each difficulties as difficulty}
          <option value={difficulty}>{difficulty}</option>
        {/each}
      </select>

      <button onclick={() => set_puzzle_and_reset(selected_difficulty)}>New sudoku</button>
    </div>

    {#key isSolved}
    <div class="check_solution">
      <button onclick={() => check_is_solved()}>Check solution</button>
      {isSolved ? "Solved!" : "Unsolved"}
    </div>
    {/key}

    <div class="reveal_solution">
      <button onclick={() => reveal_solution()}>Reveal solution</button>
    </div>

  </div>
  
</main>


<style>

.board_container{
  justify-content: centre;
}

.board{
    display: grid;
    grid-template-columns: repeat(9, auto);
    grid-template-rows: repeat(9, auto);
    padding: auto;
    justify-content: center;
}

.board_item{
  width: 80px;
  height: 80px;
  border-color: white;
  border-style: dashed;
  border-width: 1px;
  font-size: x-large;
  text-align: center;
  align-content: center;
  color: rgb(50, 205, 100);
  background-color: #2f2f2f;
}

.board_item_input {
  width: 50px;
  height: 50px;
  color: #f6f6f6;
  background-color: #2f2f2f;
  text-align: center;
  font-size: x-large;
  border-style: none;
}

.board_item_input:hover{
  background-color: rgba(50, 205, 100, 0.1);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}
</style>