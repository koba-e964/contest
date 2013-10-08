#include <stdio.h>
#include <vector>
#include <algorithm>

using namespace std;

#define RMAX 500
int tid[RMAX];
int pid[RMAX];
int time[RMAX];
int ok[RMAX];

#define TMAX 50
#define PMAX 10


int pen[TMAX][PMAX];
int pent[TMAX];
int cor[TMAX];


#define TEST 0

struct Dat{
	int team;
	int cor;
	int pent;
	Dat(int team,int cor,int pent):team(team),cor(cor),pent(pent){}
	bool operator<(const Dat& ano)const{
		if(cor!=ano.cor)return cor>ano.cor;
		if(pent!=ano.pent)return pent<ano.pent;
		return team<ano.team;
	}
};


int main(void){
	int t,p,r;
	while(scanf("%d%d%d",&t,&p,&r)==3&&(t||p||r)){
		for(int i=0;i<r;i++){
			scanf("%d%d%d",tid+i,pid+i,time+i);
			int ch;
			while((ch=getchar())<=' '){}
			ok[i]=ch=='C'?1:0;
			while(getchar()>' '){}
		}
		if(TEST){
			for(int i=0;i<r;i++){
				printf("%d\n",ok[i]);
			}
		}
		for(int i=0;i<t;i++){
			pent[i]=0;
			cor[i]=0;
			for(int j=0;j<p;j++){
				pen[i][j]=0;
			}
		}
		for(int i=0;i<r;i++){
			int team=tid[i]-1;
			int prob=pid[i]-1;
			if(ok[i]){
				cor[team]++;
				pent[team]+=pen[team][prob]+time[i];
			}else{
				pen[team][prob]+=1200;
			}
		}
		vector<Dat> vec;
		for(int i=0;i<t;i++){
			vec.push_back(Dat(i,cor[i],pent[i]));
		}
		sort(vec.begin(),vec.end());
		for(int i=0;i<vec.size();i++){
			printf("%d %d %d\n",vec[i].team+1,vec[i].cor,vec[i].pent);
		}
	}
	return 0;
}
