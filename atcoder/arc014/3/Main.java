import java.util.*;

/**
	ARC014-3
*/
public class Main{
	static int n;
	static String s;
	public static void main(String[] args){
		Scanner scan=new Scanner(System.in);
		n=Integer.parseInt(scan.nextLine());
		s=scan.nextLine();
		if(n<=15){
			small();
		}else{
			return;
		}
	}
	/**
		n<=15
	*/
	static public void small(){
		int min=Integer.MAX_VALUE;
		for(int i=0;i<1<<n;i++){
			StringBuilder sb=new StringBuilder();
			for(int j=0;j<n;j++){
				char t=s.charAt(j);
				if((i&(1<<j))!=0){
					if(sb.length()>=1 && sb.charAt(0)==t){
						sb.deleteCharAt(0);
					}else{
						sb.insert(0,t);
					}
				}else{
					if(sb.length()>=1 && sb.charAt(sb.length()-1)==t){
						sb.deleteCharAt(sb.length()-1);
					}else{
						sb.append(t);
					}
				}
			}
			min=Math.min(min,sb.length());
		}
		System.out.println(min);
	}
}
