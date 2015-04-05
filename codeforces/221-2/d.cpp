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

const int N=5000;
int n,m;
int acc[N][N+1];
int run[N][N+1];
void dp(int i,int j){
	if(j==m){
		run[i][m]=0;
		return;
	}
	if(acc[i][j]==0){
		run[i][j]=0;//0
		return;
	}
	run[i][j]=run[i][j+1]+1;
}
bool debug=false;
char s[N+10];
int main(void){
	cin>>n>>m;
	REP(i,0,n){
		scanf("%s",s);
		REP(j,0,m){
			acc[i][j]=(s[j]-'0');
		}
		for(int j=m;j>=0;j--)
			dp(i,j);
	}
	int ma=0;
	if(debug){
		cout<<"run:"<<endl;
		REP(i,0,n){
			REP(j,0,m){
				cout<<run[i][j]<<" ";
			}
			cout<<endl;
		}
		cout<<"acc:"<<endl;
		REP(i,0,n){
			REP(j,0,m){
				cout<<acc[i][j]<<" ";
			}
			cout<<endl;
		}
	}
	REP(j,0,m){
		int cnt=0;
		vector<int> ar;
		ar.reserve(n);
		REP(i,0,n){
			ar.push_back(run[i][j]);
		}
		sort(ar.begin(),ar.end());
		if(debug){
			cout<<"j="<<j<<endl;
			REP(i,0,n)cout<<ar[i]<<" ";
			cout<<endl;
		}
		REP(i,0,n){
			ma=max(ma,(n-i)*ar[i]);
		}
	}
	cout<<ma<<endl;
	
}
