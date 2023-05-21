import { h } from "preact";
import Searchbox from "./searchbox/searchbox";
import { Main, Title } from "./app.styled";

const App = () => (
  <div id="app">
    <Main>
      <Title>PageRank</Title>
      <Searchbox />
    </Main>
  </div>
);

export default App;
