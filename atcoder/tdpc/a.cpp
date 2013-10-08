#include <iostream>
#include <algorithm>
using namespace std;
const int N=100;
int p[N];
const int S=101*N;
int tmp[S];

int main(void){
	int n;
	cin>>n;
	for(int i=0;i<n;i++){
		cin>>p[i];
	}
	fill_n(tmp,S,0);
	tmp[0]=1;
	for(int i=0;i<n;i++){
		int v=p[i];
		for(int s=S-v-1;s>=0;s--){
			if(tmp[s])tmp[s+v]=1;
		}
	}
	int c=0;
	for(int s=0;s<S;s++)c+=tmp[s]?1:0;
	cout<<c<<endl;
	return 0;
}