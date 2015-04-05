#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>
#include <bitset>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef bitset<150> bb;

const int N=150;

int e[150][150];
int ind[N];
pair<bool,bb> find_clique(int len,int cur,bb found){
	REP(i,0,N){
		if(i==cur)continue;
		if(e[cur][i]==0 && found[i]){ //not clique
			return pair<bool,bb>(false,bb());
		}
		if(e[cur][i] && ! found[i] && len>=1){
			found[i]=true;
			pair<bool,bb> val=find_clique(len-1,i,found);
			found[i]=false;
			if(val.first){
				return val;
			}
		}
	}
	return pair<bool,bb>(true,found);
}

int main(void){
	REP(i,0,N){
		REP(j,0,N){
			e[i][j]=i==j?0:1;
		}
		ind[i]=N-1;
	}
	int num=0;
	REP(time,0,40){
		int maxi=0;
		REP(i,1,N){
			if(ind[maxi]<ind[i])maxi=i;
		}
		cout<<"ind["<<maxi<<"]="<<ind[maxi]<<endl;
		bb inst;
		inst[maxi]=true;
		pair<bool,bb> res=find_clique(6,maxi,inst);
		cout<<time<<":";
		if(res.first){
			REP(i,0,N){
				if(res.second[i])cout<<i<<" ";
			}
			cout<<endl;
			bb b=res.second;
			REP(i,0,N){
				if(!b[i])continue;
				num++;
				REP(j,0,N){
					if(b[j]){
						e[i][j]=0;
						ind[i]--;
					}
				}
			}
		}
	}
	cout<<"total:"<<num<<endl;
}
