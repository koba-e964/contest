#include <iostream>
#include <algorithm>
#include <cassert>
#include <cstdio>
#define TEST 0

using namespace std;
int n;
const int N=16;
double dp[0x10000];

#define ret(k) return dp[bits]=(k)
double rec(int bits){
	if(dp[bits]>=0)return dp[bits];
	if(bits==0)ret(0.0);
	int minx=-1;
	double minv=1.0e+10;
	for(int x=0;x<16;x++){
		int st=0;
		double sum=0;
		for(int p=-1;p<=1;p++){
			int t=1<<(x+p);
			if(x+p<0 || x+p>=16)t=0;
			if((bits&t)!=0){
				sum+=rec(bits&~t);
			}else{
				st++;
			}
		}
		if(st==3)continue;
		double res=(sum+3.0)/(3.0-st);
		if(minx==-1||minv>res){
			minx=x;
			minv=res;
		}
	}
	if(minx==-1){
		cerr<<"bits="<<hex<<bits<<endl;
		abort();
	}
	ret(minv);
}
int main(void){
	cin>>n;
	int bits=0;
	for(int i=0;i<n;i++){
		int x;
		cin>>x;
		bits|=1<<x;
	}
	fill_n(dp,0x10000,-1.0);
	rec(0xffff);
	double res=dp[bits];
	printf("%.9f\n",res);
}
