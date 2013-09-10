#include <iostream>
#include <cassert>
#include <algorithm>
using namespace std;
typedef long long int ll;
const int M=61;
double dp[2][M][M][M];
int t=0;
int main(void){
	int n;
	ll d;
	int f2=0,f3=0,f5=0;
	cin>>n>>d;
	while(d>1){ //{2,3,5}-smoothness
		if(d%2==0){
			d/=2;
			f2++;
			continue;
		}
		if(d%3==0){
			d/=3;
			f3++;
			continue;
		}
		if(d%5==0){
			d/=5;
			f5++;
			continue;
		}
		//not smooth
		cout<<0.0<<endl;
		return 0;
	}
	assert(d==1);
	fill_n(reinterpret_cast<double*>(dp),2*M*M*M,0.0);
	dp[0][0][0][0]=1.0;
	for(int i=0;i<n;i++){
		fill_n(reinterpret_cast<double*>(dp[1-t]),M*M*M,0.0);
		for(int x=0;x<=f2;x++)for(int y=0;y<=f3;y++)for(int z=0;z<=f5;z++){
			dp[1-t][x][y][z]+=dp[t][x][y][z]/6.0;
			dp[1-t][min(x+1,f2)][y][z]+=dp[t][x][y][z]/6.0;
			dp[1-t][x][min(y+1,f3)][z]+=dp[t][x][y][z]/6.0;
			dp[1-t][min(x+2,f2)][y][z]+=dp[t][x][y][z]/6.0;
			dp[1-t][x][y][min(z+1,f5)]+=dp[t][x][y][z]/6.0;
			dp[1-t][min(x+1,f2)][min(y+1,f3)][z]+=dp[t][x][y][z]/6.0;
		}
		t=1-t;
	}
	cout<<dp[t][f2][f3][f5]<<endl;
	return 0;
}