import java.io.*;
import java.util.*;

public class Sol{
	int[] reading()throws IOException{
		String filename="crossing.txt";
		Scanner scan=new Scanner(new File(filename));
		List<Integer> list=new ArrayList<Integer>();
		while(scan.hasNext()){
			String str=scan.next();
			list.add(Integer.parseInt(str));
		}
		int n=list.size();
		int[] out=new int[n];
		for(int i=0;i<n;i++){
			out[i]=list.get(i);
		}
		return out;
	}
	long solve2(int[] ary){
		int n=ary.length;
		int m=Integer.highestOneBit(n)*2;
		long ans=0;
		long[] cnt=new long[m]; // cnt[...01110000]:ary[0]~ary[i]‚ÅA[....01110000, ....10000000)‚É‘¶Ý‚·‚é‚à‚Ì‚ÌŒÂ”
		Arrays.fill(cnt,0L);
		for(int i=0;i<n;i++){
			if(i%30000==0)System.out.println(i+"/"+n);
			int v=ary[i]; // v is in [1,n]
			int w=v;
			long c=0;
			while(w<m){
				int least=w&(-w);
				c+=cnt[w];
				w+=least;
			}
			ans+=c;
			w=v;
			while(w>0){
				int least=w&(-w);
				cnt[w]++;
				w-=least;
			}
		}
		return ans;
	}

	void test()throws IOException{
		int[] ary=reading();
		//solution 2
		int[] cl=ary.clone();
		long st=System.currentTimeMillis();
		long res=solve2(cl);
		long en=System.currentTimeMillis();
		System.out.println("result="+res);
		System.out.println("time:"+((en-st)/1000.0)+" sec");
	}
	public static void main(String[] args)throws IOException{
		new Sol().test();
	}
}

