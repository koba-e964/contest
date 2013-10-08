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

const int TEST=0;

const int N=400;

using namespace std;
typedef long long int ll;


map<string,int> m;
map<int,string> inv;
int cnt=0;

int reg(string name){
	if(m.count(name)==0){
		m.insert(pair<string,int>(name,cnt));
		inv.insert(pair<int,string>(cnt,name));
		cnt++;
		return cnt-1;
	}
	return m.find(name)->second;
}

double edges[N][N];

int main(void){
	int n;
	cin>>n;
	fill_n(reinterpret_cast<int*>(edges),N*N,-1);
	REP(i,0,n){
		string l,s;
		double mm;
		cin>>l>>mm>>s;
		int h0=reg(l);
		int h1=reg(s);
		edges[h0][h1]=mm;
		edges[h1][h0]=1.0/mm;
	}
	int x=cnt;
	if(x>N)assert(0);
	REP(k,0,x){
		REP(i,0,x){
			REP(j,0,x){
				if(edges[i][k]>=0&&edges[k][j]>=0){
					if(edges[i][j]>0)continue;
					edges[i][j]=edges[i][k]*edges[k][j];
				}
			}
		}
	}
	ll maxv=0;
	int maxi=-1,maxj=-1;
	REP(i,0,x){
		REP(j,0,x){
			if(edges[i][j]<0)continue;
			if(maxv<edges[i][j]){
				maxv=(ll)(edges[i][j]+0.5);
				maxi=i;maxj=j;
			}
		}
	}
	assert(maxi>=0);
	assert(maxj>=0);
	cout<<1<<inv[maxi]<<"="<<maxv<<inv[maxj]<<endl;
}
