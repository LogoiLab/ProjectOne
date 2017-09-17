import java.util.*;

public class PartList {
  private ArrayList<Part> partList = new ArrayList<Part>();

  public void add(Part part) {
    this.partList.add(part);
  }

  public void remove(Part part) {
    this.partList.remove(part);
  }

  public Part get(int index) {
    return this.partList.get(index);
  }

  public void sortByName() {

  }

  public void sortByNumber() {

  }
}
