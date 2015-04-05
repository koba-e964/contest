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

const bool DEBUG=false;

struct dat{
	int v[26];
	int hash;
	void init(int* t){
		hash=0;
		REP(i,0,26){
			v[i]=t[i];
			hash*=0xdead;
			hash+=v[i];
		}
	}
	bool operator< (const dat&d)const{
		if(hash!=d.hash) return hash<d.hash;
		REP(i,0,26){
			if(v[i]!=d.v[i]) return v[i]<d.v[i];
		}
		return false;
	}
	bool operator==(const dat& d)const{
		if(hash!=d.hash)return false;
		REP(i,0,26){
			if(v[i]!=d.v[i])return false;
		}
		return true;
	}
};
int n,k;
string s;
map<dat,int> memo;

int main(void){
	cin>>n>>k;
	cin>>s;
	int d[26]={0};
	REP(i,0,k){
		d[s[i]-'a']++;
	}
	dat inst;
	inst.init(d);
	memo.insert(pair<dat,int>(inst,k-1));
	REP(i,k,n){
		d[s[i]-'a']++;
		d[s[i-k]-'a']--;
		inst.init(d);
		if(DEBUG)
			cout<<"ind: "<<i<<" hash: "<<inst.hash<<endl;
		if(memo.find(inst)!= memo.end()){
			int p=memo[inst];
			if(i-p>=k){
				cout<<"YES"<<endl;
				return 0;
			}
			continue;
		}
		memo.insert(pair<dat,int>(inst,i));
	}
	cout<<"NO"<<endl;
	return 0;
	
}
