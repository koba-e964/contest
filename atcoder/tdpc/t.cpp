#include <vector>
#include <string>
#include <algorithm>
#include <iostream>
#include <cstdio>
#include <cassert>
#include <cstdlib>

const int K=1001;
const int MOD=1000000007;
inline int mod(int x,int y){
	return ((x%y)+y)%y;
}
int tmp[K];

using namespace std;

int main(void){
	int k,n;
	fill_n(tmp,K,1);
	cin>>k>>n;
	tmp[k]=mod(k,MOD);
	for(int i=k+1;i<n;i++){
		int res=mod(2*tmp[mod(i-1,k+1)],MOD);
		res=mod(res-tmp[mod(i,k+1)],MOD);
		tmp[mod(i,k+1)]=res;
	}
	cout<<tmp[mod(n-1,k+1)]<<endl;
}