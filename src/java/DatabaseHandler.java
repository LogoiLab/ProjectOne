public class DatabaseHandler {
  public static PartList loadDatabase(String path){
    return Importer.importFile(path);
  }
  public static void saveDatabase(PartList partList) {
    
  }
}
