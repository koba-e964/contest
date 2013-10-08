#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>
#include <sstream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

const int V=1000001;
int vv;
int a[10]={0};
int dp[V];

void rec(int p){
	if(p<0)return;
	if(dp[p]>=-1)return;
	int ret=0;
	if(p==0)return;
	int max1=-1;
	REP(i,1,10){
		if(p<a[i])continue;
		rec(p-a[i]);
		if(dp[p-a[i]]==-1)continue;
		int d1=dp[p-a[i]]+1;
		if(max1<d1){
			max1=d1;
		}
	}
	dp[p]=max1;
}
stringstream iss;
void traceback(int v){
	if(v==0){
		return;
	}
	for(int i=9;i>=1;i--){
		if(v<a[i])continue;
		if(dp[v]-dp[v-a[i]]==1){
			iss<<i;
			traceback(v-a[i]);
			return;
		}
	}
	iss<<"NULL";
	//assert(0);
}

int main(void){
	cin>>vv;
	REP(i,1,10)
		cin>>a[i];
	fill_n(dp,V,-2);
	dp[0]=0;
	REP(i,0,vv+1)
		rec(i+1);
	/*REP(i,0,vv+1){
		cerr<<dp[i]<<" ";
	}*/
	for(int i=vv;i>=1;i--){
		if(dp[i]==-1)continue;
		traceback(i);
		cout<<iss.str()<<endl;
		return 0;
	}
	cout<<-1<<endl;
	return 0;
}
