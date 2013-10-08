import java.util.*;

public class Main{
	public static void main(String[] args){
		Set<String> words=new HashSet<String>();
		Scanner scan=new Scanner(System.in);
		int n=Integer.parseInt(scan.nextLine());
		String[] ws=new String[n];
		for(int i=0;i<n;i++){
			ws[i]=scan.nextLine();
		}
		String last=null;
		for(int i=0;i<n;i++){
			if(last==null){
				words.add(ws[0]);
				last=ws[0];
				continue;
			}
			if(words.contains(ws[i])){
				System.out.println(i%2==0?"LOSE":"WIN");
				return;
			}
			words.add(ws[i]);
			if(last.charAt(last.length()-1)!=ws[i].charAt(0)){
				System.out.println(i%2==0?"LOSE":"WIN");
				return;
			}
			last=ws[i];
		}
		System.out.println("DRAW");
	}
}