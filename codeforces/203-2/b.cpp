#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

const bool TEST=false;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

const int N=100000;
int n;
int t[N],a[N];
int ok[N];
int edges[N];
int main(void){
	cin>>n;
	REP(i,0,n)cin>>t[i];
	REP(i,0,n)cin>>a[i];
	fill_n(ok,n,1);
	fill_n(edges,n,0);
	REP(i,0,n){
		if(a[i]==0)continue;
		edges[a[i]-1]++;
	}
	REP(i,0,n)if(edges[i]>=2)ok[i]=0;
	if(TEST){
		REP(i,0,n)cout<<(ok[i]?"*":".");
		cout<<endl;
	}
	vector<int> maxv;
	int maxl=-1;
	REP(i,0,n){
		if(t[i]==0)//not hotel
			continue;
		int cur=i;
		vector<int> v;
		while(1){
			v.push_back(cur);
			if(a[cur]==0 || ok[a[cur]-1]==0)break;
			cur=a[cur]-1;
		}
		if(TEST){
			cout<<"test:["<<i<<"] ";
			for(int i=0,s=v.size();i<s;i++){
				cout<<v[i]<<" ";
			}
			cout<<endl;
		}
		if(maxl<(int)v.size()){
			maxl=v.size();
			maxv.swap(v);
		}
	}
	cout<<maxl<<endl;
	for(int s=maxv.size()-1;s>=0;s--){
		cout<<maxv[s]+1;
		if(s!=0){
			cout<<" ";
		}
	}
	cout<<endl;
}
