import { useEffect, useState, useCallback } from "preact/hooks";
import debounce from "lodash.debounce";
import ResultList from "../resultList/resultList";
import { StyledBox } from "./searchbox.styled";

const Searchbox = () => {
  const [result, setResult] = useState([]);

  const handleSearch = (e) => {
    fetch(`/api/?q=${e.target.value}`, {
      mode: "cors",
      headers: {
        "Access-Control-Allow-Origin": "*",
      },
    })
      .then((response) => {
        return response.json();
      })
      .then((data) => {
        console.log("Otrzymane dane:", data);
        setResult(data);
      })
      .catch((error) => {
        console.log("Wystąpił błąd:", error);
      });
  };

  const debouncedChangeHandler = useCallback(debounce(handleSearch, 300), []);

  return (
    <>
      <StyledBox
        type="text"
        name="wyszukaj"
        placeholder="wyszukaj"
        onChange={debouncedChangeHandler}
      />
      <ResultList result={result} />
    </>
  );
};

export default Searchbox;
