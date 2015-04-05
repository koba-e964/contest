#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

int n,m;

const int N=100;
int d[N];
int main(void){
	cin>>n>>m;
	fill_n(d,N,0);
	REP(i,0,m){
		int a,b,c;
		cin>>a>>b>>c;
		a--,b--;
		d[a]+=c;
		d[b]-=c;
	}
	int sum=0;
	REP(i,0,n){
		sum+=d[i]>0?d[i]:0;
	}
	cout<<sum<<endl;
	
}
