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

int n;
const int N=100000;
int val[N];
map<int,int> memo;
int main(void){
	cin>>n;
	REP(i,0,n){
		cin>>val[i];
	}
	sort(val,val+n);
	int mind=0x7fffffff;
	REP(i,0,n-1){
		int d=val[i+1]-val[i];
		mind=min(mind,d);
		if(memo.find(d)==memo.end()){
			memo[d]=1;
		}else
			memo[d]++;
	}
	if(n==1){
		cout<<"-1"<<endl;
		return 0;
	}
	if(n==2){
		cout<<"3"<<endl;
		int d=val[1]-val[0];
		cout<<val[0]-d<<" ";
		if(d%2==0){
			cout<<val[0]+d/2<<" ";
		}
		cout<<val[1]+d<<endl;
		return 0;
	}
	if(memo[mind]==n-1){
		cout<<"2"<<endl;
		cout<<val[0]-mind<<" "<<val[n-1]+mind<<endl;
		return 0;
	}
	if(memo[mind]==n-2 && memo[mind*2]==1){
		REP(i,0,n-1){
			if(val[i+1]-val[i]==mind*2){
				cout<<"1"<<endl;
				cout<<val[i]+mind<<endl;
				return 0;
			}
		}
		assert(0);
	}
	cout<<"0"<<endl<<endl;
	return 0;
}
