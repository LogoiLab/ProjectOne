public class Importer {
  public static Part importLine(String line) {
    String partName = line.split(",")[0];
    Long partNumber = Long.parseLong(line.split(",")[1]);
    Double listPrice = Double.parseDouble(line.split(",")[2]);
    Double salePrice = Double.parseDouble(line.split(",")[3]);
    boolean onSale = Boolean.parseBoolean(line.split(",")[4]);
    Long quantity = Long.parseLong(line.split(",")[5]);
    return new Part(partName, partNumber, listPrice, salePrice, onSale, quantity);
  }
  public static PartList importFile(String path) {
    PartList partList = new PartList();
    return partList;
  }

}
