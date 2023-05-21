import { h } from "preact";
// import { useEffect } from "preact/hooks";
import {
  Container,
  Header,
  LinkTitle,
  ValueTitle,
  Row,
  Link,
  Value,
} from "./resultList.styled";

const ResultList = ({ result }) => {
  const sorted = result.sort((a, b) => b.value - a.value);
  return (
    <Container>
      <Header>
        <LinkTitle>Pasujące strony:</LinkTitle>
        <ValueTitle>pagerank</ValueTitle>
      </Header>
      {sorted.map(({ title, url, value }) => (
        <Row>
          <Link href={url}>{title}</Link>
          <Value>{`${(value * 100).toPrecision(3)}%`}</Value>
        </Row>
      ))}
    </Container>
  );
};

export default ResultList;
