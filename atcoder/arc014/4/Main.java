import java.util.*;

/**
	ARC014-4
*/
public class Main{
	static int all,n,m;
	static int[] l,x,y;
	static int[] vals;
	static int prep[];
	public static void main(String[] args){
		Scanner scan=new Scanner(System.in);
		String first=scan.nextLine();
		String[] split=first.split("\\s");
		all=Integer.parseInt(split[0]);
		n=Integer.parseInt(split[1]);
		m=Integer.parseInt(split[2]);
		l=new int[n];
		for(int i=0;i<n;i++){l[i]=Integer.parseInt(scan.nextLine());}
		x=new int[m];
		y=new int[m];
		for(int i=0;i<m;i++){
			String[] ar=scan.nextLine().split("\\s");
			x[i]=Integer.parseInt(ar[0]);
			y[i]=Integer.parseInt(ar[1]);
		}
		vals=new int[n-1];
		int c=0;
		for(int i=0;i<n-1;i++){
			vals[i]=l[i+1]-l[i]-1;
		}
		Arrays.sort(vals);
		prep=new int[n-1];
		for(int i=0;i<n-1;i++){
			c+=vals[i];
			prep[i]=c;
		}
		for(int i=0;i<m;i++){
			serve2(x[i],y[i]);
		}
	}
	static void serve(int x,int y){
		int count=0;
		int min=1;
		int sup=1;
		for(int i=0;i<n;i++){
			int start=Math.max(1,l[i]-x);
			int end=Math.min(l[i]+y+1,all+1);
			if(sup>start){//overlaps
				count+=end-sup;
				sup=end;
			}else{
				count+=end-start;
				min=start;
				sup=end;
			}
		}
		System.out.println(count);
	}
	static void serve2(int x,int y){
		int count=n+Math.min(l[0]-1,x)+Math.min(all-l[n-1],y);
		int bs=Arrays.binarySearch(vals,x+y);
		if(bs<0){
			bs=-bs-1;
		}
		count+=bs>=1?prep[bs-1]:0;
		count+=(x+y)*(n-bs-1);
		System.out.println(count);
	}
}