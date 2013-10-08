#include <iostream>
#include <vector>
#include <algorithm>
#include <string>
#include <cassert>

#define TEST 0

using namespace std;
const int MOD=1000000007;
const int D=101;
const int N=10001;
inline int mod(int x,int y){
	return ((x%y)+y)%y;
}
int dp[N][D][11];//dp[nn][dd][x]: x*10^nn–¢–ž‚ÅŠeˆÊ‚ÌŒ…‚Ì˜a‚ªdd mod d‚Ì‚à‚Ì
int main(void){
	int d;
	string sn;
	cin>>d>>sn;
	int n=sn.length();
	fill_n(reinterpret_cast<int*>(dp),N*D*11,0);
	for(int y=1;y<11;y++){
		for(int z=0;z<y;z++){
			dp[0][mod(z,d)][y]+=1;
		}
	}
		if(TEST){
			cout<<"nn="<<0<<endl;
			for(int dd=0;dd<d;dd++){
				cout<<dd<<": ";
				for(int x=1;x<11;x++){
					cout<<dp[0][dd][x]<<" ";
				}
				cout<<endl;
			}
		}
	for(int nn=1;nn<n;nn++){
		for(int x=1;x<11;x++){
			if(x>=1){
				for(int i=0;i<d;i++){
					dp[nn][i][x]=dp[nn][i][x-1];
				}
			}
			for(int dd=0;dd<d;dd++){
				dp[nn][mod(dd+x-1,d)][x]+=dp[nn-1][dd][10];
				dp[nn][mod(dd+x-1,d)][x]%=MOD;
			}
		}
		if(TEST){
			cout<<"nn="<<nn<<endl;
			for(int dd=0;dd<d;dd++){
				cout<<dd<<": ";
				for(int x=1;x<11;x++){
					cout<<dp[nn][dd][x]<<" ";
				}
				cout<<endl;
			}
		}
	}
	int cnt=0;
	int rem=0;
	for(int i=0;i<n;i++){
		assert(rem>=0);
		int dig=sn[i]-'0';
		cnt+=dp[n-1-i][rem][dig];
		cnt%=MOD;
		rem=mod(rem-dig,d);
	}
	cnt+=rem%d==0?1:0;
	cnt--;//zero
	cnt=mod(cnt,MOD);
	cout<<cnt<<endl;
}
