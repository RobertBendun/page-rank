import styled from "styled-components";

export const Container = styled.div`
  width: 350px;
  height: 100%;
  overflow-y: auto;
`;

export const Header = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  width: 100%;
  padding-bottom: 10px;
  margin: 20px 0;
  border-bottom: 3px solid #dde6ed;
`;

export const LinkTitle = styled.p`
  flex-grow: 7;
  color: #dde6ed;
  font-weight: bold;
`;

export const ValueTitle = styled.p`
  flex-grow: 1;
  text-align: center;
  color: #dde6ed;
  font-weight: bold;
`;

export const Row = styled(Header)`
  padding-bottom: 5px;
  margin: 5px 0;
  border-bottom: 1px solid #dde6ed;
`;

export const Link = styled.a`
  flex-grow: 7;
  text-decoration: none;
  color: #dde6ed;
`;

export const Value = styled.div`
  flex-grow: 1;
  color: #dde6ed;
  text-align: center;
`;
