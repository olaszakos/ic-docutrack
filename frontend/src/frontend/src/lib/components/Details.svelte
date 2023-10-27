<script>
  import { Row, Col, Table } from "sveltestrap";

  export let file = {};
</script>

<Row>
  <Col class="col-md-6">
    <Table borderless>
      <tbody>
        <tr>
          <th scope="row">File Name</th>
          <td>{file.name}</td>
        </tr>
      </tbody>
    </Table>
  </Col>
</Row>
