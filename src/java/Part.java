public class Part {
  private String partName;
  private Long partNumber;
  private Double listPrice;
  private Double salePrice;
  private boolean onSale;
  private Long quantity;

  public Part(String partName, Long partNumber, Double listPrice, Double salePrice, boolean onSale, Long quantity) {
    this.partName = partName;
    this.partNumber = partNumber;
    this.listPrice = listPrice;
    this.salePrice = salePrice;
    this.onSale = onSale;
    this.quantity = quantity;
  }

  public String getPartName() {
    return this.partName;
  }

  public Long getPartNumber() {
    return this.partNumber;
  }
  public Double getListPrice() {
    return this.listPrice;
  }
  public Double getSalePrice() {
    return this.salePrice;
  }
  public boolean isOnSale() {
    return this.onSale;
  }
  public Long getQuantity() {
    return this.quantity;
  }

  public void setPartName(String partName) {
    this.partName = partName;
  }

  public void setPartNumber(Long partNumber) {
    this.partNumber = partNumber;
  }
  public void setListPrice(Double listPrice) {
    this.listPrice = listPrice;
  }
  public void setSalePrice(Double salePrice) {
    this.salePrice = salePrice;
  }
  public void setOnSale(boolean onSale) {
    this.onSale = onSale;
  }
  public void setQuantity(Long quantity) {
    this.quantity = quantity;
  }

  public void decrement() {
    this.quantity--;
  }

  public void decrement(int amt) {
    this.quantity = this.quantity - amt;
  }

  public void increment() {
    this.quantity++;
  }

  public void increment(int amt) {
    this.quantity = this.quantity + amt;
  }

}
